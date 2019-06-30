use std::thread;

use mongodb::oid::ObjectId;
use mongodb::{bson, doc};
use chrono::Utc;

use crate::util::Collection;
use crate::util::Database;
use crate::util::Error;

use super::manga_data::MangaData;
use super::genre::Genre;
use super::dmk;

// Typedef
type FetchGenreHandle = thread::JoinHandle<Result<Vec<MangaData>, Error>>;
type FetchMangaHandle = thread::JoinHandle<Result<MangaData, Error>>;

#[collection("manga")]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Manga {
  #[serde(rename="_id")]
  id: ObjectId,

  // Book keeping fields
  add_date_time: bson::UtcDateTime, // The time manga initially get to be stored
  update_date_time: bson::UtcDateTime, // The time manga last changed
  refresh_date_time: bson::UtcDateTime, // The time we do the refresh

  #[serde(flatten)]
  data: MangaData,
}

impl Manga {
  pub fn new(data: &MangaData) -> Result<Self, Error> {
    let now = mongodb::UtcDateTime::from(Utc::now());
    Ok(Self {
      id: ObjectId::new().map_err(|_| Error::CannotCreateObjectId)?,
      add_date_time: now,
      update_date_time: now,
      refresh_date_time: now,
      data: data.clone()
    })
  }

  pub fn add_date_time(&self) -> &bson::UtcDateTime {
    &self.add_date_time
  }

  pub fn update_date_time(&self) -> &bson::UtcDateTime {
    &self.update_date_time
  }

  pub fn refresh_date_time(&self) -> &bson::UtcDateTime {
    &self.refresh_date_time
  }

  pub fn data(&self) -> &MangaData {
    &self.data
  }

  pub fn get_by_dmk_id(conn: &Database, dmk_id: &String) -> Result<Option<Self>, Error> {
    Self::get_one(conn, Some(doc! { "dmk_id": dmk_id }), None)
  }

  pub fn get_or_fetch_by_dmk_id(conn: &Database, dmk_id: &String) -> Result<Self, Error> {
    Self::get_by_dmk_id(conn, dmk_id).and_then(|maybe_self| match maybe_self {
      Some(manga) => Ok(manga),
      None => match dmk::fetch_manga_data(dmk_id) {
        Ok(data) => Self::insert(conn, &data),
        Err(err) => Err(err)
      }
    })
  }

  pub fn insert(conn: &Database, data: &MangaData) -> Result<Self, Error> {
    let coll = Self::coll(&conn);
    let wrapped = Self::new(data)?;
    match coll.insert_one(wrapped.to_doc()?, None) {
      Ok(result) => match result.inserted_id {
        Some(_) => Ok(wrapped),
        None => Err(Error::MangaExistedError)
      },
      Err(_) => Err(Error::DatabaseError)
    }
  }

  pub fn upsert(conn: &Database, data: &MangaData) -> Result<Self, Error> {
    let dmk_id = data.dmk_id();
    match Self::get_by_dmk_id(conn, dmk_id)? {
      Some(manga) => {
        if manga.data == *data {
          Self::touch(conn, dmk_id)
        } else {
          let coll = Self::coll(&conn);
          match coll.find_one_and_update(doc! {
            "dmk_id": dmk_id
          }, doc! {
            "$set": bson::to_bson(&data).map_err(|_| Error::SerializeError)?,
            "$currentDate": {
              "refresh_date_time": true,
              "update_date_time": true,
            }
          }, None) {
            Ok(result) => match result {
              Some(doc) => Self::from_doc(doc),
              None => Err(Error::MangaNotFoundError)
            },
            Err(_) => Err(Error::DatabaseError)
          }
        }
      },
      None => Self::insert(conn, data)
    }
  }

  pub fn touch(conn: &Database, dmk_id: &String) -> Result<Self, Error> {
    let coll = Self::coll(&conn);
    match coll.find_one_and_update(doc! {
      "dmk_id": dmk_id
    }, doc! {
      "$currentDate": { "refresh_date_time": true }
    }, None) {
      Ok(result) => match result {
        Some(doc) => Self::from_doc(doc),
        None => Err(Error::MangaNotFoundError)
      },
      Err(_) => Err(Error::DatabaseError)
    }
  }

  pub fn setup_collection_index(conn: &Database) -> Result<(), Error> {
    let coll = Self::coll(&conn);
    match coll.create_index(doc! {
      "dmk_id": 1
    }, Some(mongodb::coll::options::IndexOptions {
      unique: Some(true),
      ..Default::default()
    })) {
      Ok(_) => Ok(()),
      Err(_) => Err(Error::DatabaseError),
    }
  }

  pub fn total_amount(conn: &Database) -> Result<i64, Error> {
    let coll = Self::coll(&conn);
    coll.count(None, None).map_err(|_| Error::DatabaseError)
  }

  /// Fetch latest manga of the given genres
  pub fn fetch_latest(conn: &Database, genres: Vec<&'static Genre>) -> Result<Vec<Self>, Error> {

    // Ver.3 Double Layer Parallel
    let genre_handles : Vec<FetchGenreHandle> = genres.into_iter().map(|genre| {
      thread::spawn(move || -> Result<Vec<MangaData>, Error> {
        let ids = dmk::fetch_latest_manga_with_genre(genre)?;
        let manga_handles : Vec<FetchMangaHandle> = ids.into_iter().take(20).map(|manga| {
          thread::spawn(move || dmk::fetch_manga_data(&manga.0))
        }).collect();
        Ok(manga_handles.into_iter().filter_map(|handle| {
          handle.join().ok().and_then(|res| match res {
            Ok(manga) => Some(manga),
            Err(err) => { println!("Error {}: {}", err.code(), err.msg()); None }
          })
        }).collect())
      })
    }).collect();

    let genre_mangas : Vec<Self> = genre_handles.into_iter().filter_map(|handle| -> Option<Vec<Self>> {
      handle.join().ok().and_then(|res| match res {
        Ok(genre_mangas) => Some(genre_mangas.into_iter().filter_map(|data: MangaData| -> Option<Self> {
          Manga::upsert(conn, &data).ok()
        }).collect()),
        Err(err) => { println!("Error {}: {}", err.code(), err.msg()); None }
      })
    }).flatten().collect();

    Ok(genre_mangas)

    // Ver.1, Async on fetching part
    // let latest_manga_ids : Vec<String> = dmk::fetch_latest_manga()?;
    // let _ = Self::insert(conn, genre, &latest_manga_ids)?;
    // let handles : Vec<FetchMangaHandle> = latest_manga_ids.into_iter().map(|id| {
    //   thread::spawn(move || dmk::fetch_manga_data(&id))
    // }).collect();
    // let manga_datas : Vec<MangaData> = handles.into_iter().filter_map(|handle| {
    //   handle.join().ok().and_then(|res| res.ok())
    // }).collect();
    // let mangas : Vec<Manga> = manga_datas.into_iter().filter_map(|data| {
    //   Manga::upsert(conn, &data).ok()
    // }).collect();
    // Ok(mangas)

    // Ver.2, Non-async way
    // let latest_manga_ids : Vec<String> = dmk::fetch_latest_manga()?;
    // let _ = Self::insert(conn, genre, &latest_manga_ids)?;
    // let mangas : Vec<Manga> = latest_manga_ids.into_iter().filter_map(|id| {
    //   let data = dmk::fetch_manga_data(&id).ok()?;
    //   Manga::upsert(conn, &data).ok()
    // }).collect();
    // Ok(mangas)
  }

  /// Fetch the latest manga of all the other genres
  pub fn fetch_all_genres(conn: &Database) -> Result<Vec<Self>, Error> {
    Self::fetch_latest(conn, Genre::all_genres())
  }

  /// Fetch the latest manga of the "all" genre (basically the overall genre)
  pub fn fetch_overall(conn: &Database) -> Result<Vec<Self>, Error> {
    Self::fetch_latest(conn, vec![Genre::all()])
  }

  pub fn fetch_ended(conn: &Database) -> Result<Vec<Self>, Error> {
    let ids : Vec<(String, String)> = dmk::fetch_ended()?;
    Ok(ids.chunks(50).map(|chunk: &[(String, String)]| -> Vec<Self> { // A single chunk has size of 50
      let handles : Vec<FetchMangaHandle> = chunk.to_owned().into_iter().map(|manga| {
        thread::spawn(move || dmk::fetch_manga_data(&manga.0))
      }).collect();
      handles.into_iter().filter_map(|handle| {
        let manga_data = handle.join().ok().and_then(|res| match res {
          Ok(manga) => Some(manga),
          Err(err) => { println!("Error {}: {}", err.code(), err.msg()); None }
        })?;
        Self::upsert(conn, &manga_data).ok()
      }).collect()
    }).flatten().collect())
  }

  pub fn fetch_oldest_updating(conn: &Database, amount: i64) -> Result<Vec<Self>, Error> {
    let coll = Self::coll(&conn);

    // Type definition of the database result for deserialization
    #[derive(Deserialize)]
    struct DatabaseResult {
      dmk_id: String
    };

    // First get the oldest manga dmk_ids from the database
    let dmk_ids = coll.find(Some(doc! {
      "status": "updating",
    }), Some(mongodb::coll::options::FindOptions {
      sort: Some(doc! {
        "update_date_time": 1,
      }),
      limit: Some(amount),
      projection: Some(doc! {
        "dmk_id": 1,
      }),
      ..Default::default()
    })).map_err(|_| Error::DatabaseError)?.map(|result| match result {
      Ok(doc) => bson::from_bson::<DatabaseResult>(mongodb::Bson::Document(doc)).map_err(|_| Error::DeserializeError).map(|r| r.dmk_id),
      Err(_) => Err(Error::DatabaseError)
    }).filter_map(Result::ok).collect::<Vec<_>>();

    // Turn each dmk_id into a thread handle
    let handles : Vec<FetchMangaHandle> = dmk_ids.into_iter().map(|dmk_id| {
      thread::spawn(move || dmk::fetch_manga_data(&dmk_id))
    }).collect();

    // Join all the handle and upsert them into mongodb
    Ok(handles.into_iter().filter_map(|handle| {
      let manga_data = handle.join().ok().and_then(|res| match res {
        Ok(manga) => Some(manga),
        Err(err) => { println!("Error {}: {}", err.code(), err.msg()); None }
      })?;
      Self::upsert(conn, &manga_data).ok()
    }).collect())
  }

  pub fn get_latest_10(conn: &Database, genre: Option<&'static Genre>) -> Result<Vec<Self>, Error> {
    Self::get(conn, match genre {
      Some(genre) => Some(doc!{ "genre": genre.id }),
      None => None,
    }, Some(mongodb::coll::options::FindOptions {
      sort: Some(doc! {
        "update_date_time": -1,
      }),
      limit: Some(10),
      ..Default::default()
    }))
  }

  pub fn search(conn: &Database, text: &String) -> Result<Vec<Self>, Error> {
    Self::get(conn, Some(doc! {
      "title": {
        "$regex": format!(".*{}.*", text)
      }
    }), None)
  }
}