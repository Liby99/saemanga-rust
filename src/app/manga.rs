use mongodb::oid::ObjectId;
use mongodb::{bson, doc};
use chrono::Utc;

use super::manga_data::MangaData;
use crate::util::Collection;
use crate::util::Database;
use crate::util::Error;

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
    Self::get_one(&conn, Some(doc! { "dmk_id": dmk_id }), None)
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

  pub fn upsert(conn: &Database, data: &MangaData) -> Result<bool, Error> {
    let dmk_id = data.dmk_id();
    match Self::get_by_dmk_id(conn, dmk_id)? {
      Some(manga) => {
        if manga.data == *data {
          println!("Same...!");
          Self::touch(conn, dmk_id).map(|_| false)
        } else {
          println!("Not Same...!");
          let coll = Self::coll(&conn);
          match coll.update_one(doc! { "dmk_id": dmk_id }, doc! {
            "$set": bson::to_bson(&data).map_err(|_| Error::SerializeError)?,
            "$currentDate": {
              "refresh_date_time": true,
              "update_date_time": true,
            }
          }, None) {
            Ok(result) => Ok(result.modified_count == 1),
            Err(_) => Err(Error::DatabaseError)
          }
        }
      },
      None => {
        Self::insert(conn, data)?;
        Ok(true)
      }
    }
  }

  pub fn touch(conn: &Database, dmk_id: &String) -> Result<(), Error> {
    let coll = Self::coll(&conn);
    match coll.update_one(doc! { "dmk_id": dmk_id }, doc! {
      "$currentDate": {
        "refresh_date_time": true
      }
    }, None) {
      Ok(_) => Ok(()),
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
}