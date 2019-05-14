use mongodb::oid::ObjectId;
use mongodb::{Bson, bson, doc};
use chrono::Utc;
use std::thread;

use crate::util::Database;
use crate::util::collection::Collection;
use crate::util::error::Error;
use super::manga_data::MangaData;
use super::manga::Manga;
use super::genre::Genre;
use super::dmk;

#[collection("latest")]
#[derive(Serialize, Deserialize)]
pub struct LatestManga {
  #[serde(rename="_id")]
  id: ObjectId,
  genre: Option<&'static Genre>, // If none then general update
  manga_dmk_ids: Vec<String>,
  update_date_time: bson::UtcDateTime,
}

type FetchMangaHandle = thread::JoinHandle<Result<MangaData, Error>>;

impl LatestManga {
  pub fn fetch_latest(conn: &Database) -> Result<Vec<Manga>, Error> {
    let latest_manga_ids : Vec<String> = dmk::fetch_latest_manga()?;
    // let _ = Self::insert(conn, None, &latest_manga_ids)?;
    let handles : Vec<FetchMangaHandle> = latest_manga_ids.into_iter().map(|id| {
      thread::spawn(move || dmk::fetch_manga_data(&id))
    }).collect();
    let manga_datas : Vec<MangaData> = handles.into_iter().filter_map(|handle| {
      handle.join().ok().and_then(|res| res.ok())
    }).collect();
    let mangas : Vec<Manga> = manga_datas.into_iter().filter_map(|data| {
      Manga::upsert(conn, &data).ok()
    }).collect();
    Ok(mangas)
  }

  pub fn new(genre: Option<&'static Genre>, manga_dmk_ids: &Vec<String>) -> Result<Self, Error> {
    Ok(Self {
      id: ObjectId::new().map_err(|_| Error::CannotCreateObjectId)?,
      genre,
      manga_dmk_ids: manga_dmk_ids.clone(),
      update_date_time: bson::UtcDateTime::from(Utc::now()),
    })
  }

  pub fn insert(conn: &Database, genre: Option<&'static Genre>, manga_dmk_ids: &Vec<String>) -> Result<Self, Error> {
    let coll = Self::coll(conn);
    let latest = Self::new(genre, manga_dmk_ids)?;
    match coll.update_one(doc! {
      "genre": match genre {
        Some(g) => bson::to_bson(g.id).map_err(|_| Error::SerializeError)?,
        None => Bson::Null
      },
    }, doc! {
      "$set": latest.to_doc()?
    }, Some(mongodb::coll::options::UpdateOptions {
      upsert: Some(true),
      ..Default::default()
    })) {
      Ok(result) => match result.modified_count > 0 || result.upserted_id.is_some() {
        true => Ok(latest),
        _ => Err(Error::NoneInsertedError),
      },
      Err(_) => Err(Error::DatabaseError),
    }
  }
}