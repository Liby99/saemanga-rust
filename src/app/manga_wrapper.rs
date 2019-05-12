use mongodb::oid::ObjectId;
use mongodb::coll::Collection;
use mongodb::ordered::OrderedDocument;
use mongodb::{Bson, bson, doc};
use chrono::Utc;

use super::error::Error;
use super::manga::Manga;
use crate::util::database::Database;

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
  pub fn coll(conn: &Database) -> Collection {
    conn.collection("manga")
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

  pub fn from_bson(bs: Bson) -> Result<Self, Error> {
    match bson::from_bson::<Self>(bs) {
      Ok(user) => Ok(user),
      Err(_) => Err(Error::DeserializeError)
    }
  }

  pub fn from_doc(doc: OrderedDocument) -> Result<Self, Error> {
    Self::from_bson(bson::Bson::Document(doc))
  }

  pub fn to_bson(&self) -> Result<Bson, Error> {
    match bson::to_bson(&self) {
      Ok(bs) => Ok(bs),
      Err(_) => Err(Error::SerializeError),
    }
  }

  pub fn to_doc(&self) -> Result<OrderedDocument, Error> {
    self.to_bson().and_then(|bs| match bs {
      Bson::Document(doc) => Ok(doc),
      _ => Err(Error::SerializeError),
    })
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

  pub fn get_all(conn: &Database) -> Result<Vec<Self>, Error> {
    let coll = Self::coll(&conn);
    let cursor = coll.find(None, None).map_err(|_| Error::DatabaseError)?;
    let mangas = cursor.map(|result| match result {
      Ok(doc) => Self::from_doc(doc),
      Err(_) => Err(Error::DatabaseError)
    }).filter_map(Result::ok).collect::<Vec<_>>();
    Ok(mangas)
  }

  pub fn get(conn: &Database, dmk_id: &String) -> Result<Self, Error> {
    let coll = Self::coll(&conn);
    let option_user_doc = coll.find_one(Some(doc! {
      "dmk_id": dmk_id
    }), None).map_err(|_| Error::DatabaseError)?;
    match option_user_doc {
      Some(user_doc) => Self::from_doc(user_doc),
      None => Err(Error::MangaNotFoundError)
    }
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