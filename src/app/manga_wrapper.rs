use mongodb::oid::ObjectId;
use mongodb::{bson, doc};
use chrono::Utc;

use crate::util::database::Database;
use super::manga::Manga;
use super::error::Error;

#[collection("manga")]
#[derive(Debug, Serialize, Deserialize)]
pub struct MangaWrapper {
  #[serde(rename="_id")]
  id: ObjectId,

  // Book keeping fields
  add_date_time: bson::UtcDateTime,
  update_date_time: bson::UtcDateTime,
  refresh_date_time: bson::UtcDateTime,

  #[serde(flatten)]
  manga: Manga,
}

impl MangaWrapper {
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

  pub fn new(manga: &Manga) -> Result<MangaWrapper, Error> {
    let now = mongodb::UtcDateTime::from(Utc::now());
    Ok(MangaWrapper {
      id: ObjectId::new().map_err(|_| Error::CannotCreateObjectId)?,
      add_date_time: now,
      update_date_time: now,
      refresh_date_time: now,
      manga: manga.clone()
    })
  }

  pub fn get_by_dmk_id(conn: &Database, dmk_id: &String) -> Result<Self, Error> {
    Self::get_one(&conn, Some(doc! {
      "dmk_id": dmk_id
    }), None).and_then(|res| res.ok_or(Error::MangaNotFoundError))
  }

  pub fn insert(conn: &Database, manga: &Manga) -> Result<Self, Error> {
    let coll = Self::coll(&conn);
    let wrapped = Self::new(manga)?;
    println!("{:?}", wrapped);
    match coll.insert_one(wrapped.to_doc()?, None) {
      Ok(result) => match result.inserted_id {
        Some(_) => Ok(wrapped),
        None => Err(Error::MangaExistedError)
      },
      Err(_) => Err(Error::DatabaseError)
    }
  }

  pub fn manga(&self) -> &Manga {
    &self.manga
  }
}