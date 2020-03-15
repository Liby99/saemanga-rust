use chrono::Utc;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use lazy_static::lazy_static;
use mongodb::oid::ObjectId;
use mongodb::{bson, doc};
use regex::Regex;

use crate::util::Collection;
use crate::util::Database;
use crate::util::Error;

pub fn encrypt_password(password: &String) -> String {
  let mut hasher = Sha256::new();
  hasher.input_str(&password.as_str());
  hasher.result_str()
}

#[collection("user")]
#[derive(Serialize, Deserialize)]
pub struct User {
  #[serde(rename = "_id")]
  id: ObjectId,
  display_name: String,
  username: String,
  password: String,
  is_admin: bool,
  visit_count: i32,
  register_date_time: bson::UtcDateTime,
  last_visit_date_time: bson::UtcDateTime,
}

impl User {
  pub fn new(username: &String, password: &String) -> Result<Self, Error> {
    if Self::is_valid_password(&password) {
      if Self::is_valid_username(&username) {
        let hashed_pwd = encrypt_password(&password);
        let now = mongodb::UtcDateTime::from(Utc::now());
        Ok(User {
          id: ObjectId::new().map_err(|_| Error::CannotCreateObjectId)?,
          display_name: username.clone(),
          username: username.to_lowercase(), // Always use lower case to store username
          password: hashed_pwd,
          is_admin: false,
          visit_count: 0,
          register_date_time: now,
          last_visit_date_time: now,
        })
      } else {
        Err(Error::InvalidUsername)
      }
    } else {
      Err(Error::InvalidPassword)
    }
  }

  pub fn id(&self) -> &ObjectId {
    &self.id
  }

  pub fn display_name(&self) -> &String {
    &self.display_name
  }

  pub fn username(&self) -> &String {
    &self.username
  }

  pub fn register_date_time(&self) -> &bson::UtcDateTime {
    &self.register_date_time
  }

  pub fn last_visit_date_time(&self) -> &bson::UtcDateTime {
    &self.last_visit_date_time
  }

  pub fn visit_count(&self) -> i32 {
    self.visit_count
  }

  pub fn is_admin(&self) -> bool {
    self.is_admin
  }

  pub fn is_valid_username(username: &String) -> bool {
    lazy_static! {
      static ref USERNAME_REG: Regex = Regex::new(r"^[A-Za-z0-9\-_\[\]]{4,16}$").unwrap();
    }
    USERNAME_REG.is_match(&username.as_str())
  }

  pub fn is_valid_password(password: &String) -> bool {
    lazy_static! {
      static ref PASSWORD_REG: Regex = Regex::new(r"^[A-Za-z0-9\-_\.#*@]{8,32}$").unwrap();
    }
    PASSWORD_REG.is_match(&password.as_str())
  }

  pub fn is_password_match(&self, password: &String) -> bool {
    let hashed_pwd = encrypt_password(password);
    hashed_pwd == self.password
  }

  pub fn get_by_oid(conn: &Database, id: &ObjectId) -> Result<Self, Error> {
    Self::get_one(
      &conn,
      Some(doc! {
        "_id": id.clone(),
      }),
      None,
    )
    .and_then(|res| res.ok_or(Error::UserNotFoundError))
  }

  pub fn get_by_id(conn: &Database, id: &String) -> Result<Self, Error> {
    let oid = ObjectId::with_string(id.as_str()).map_err(|_| Error::CannotParseObjectId)?;
    Self::get_by_oid(conn, &oid)
  }

  pub fn get_by_username(conn: &Database, username: &String) -> Result<Self, Error> {
    Self::get_one(
      &conn,
      Some(doc! {
        "username": username.to_lowercase(),
      }),
      None,
    )
    .and_then(|res| res.ok_or(Error::UserNotFoundError))
  }

  pub fn insert(conn: &Database, username: &String, password: &String) -> Result<Self, Error> {
    let coll = Self::coll(&conn);
    let user = Self::new(&username, &password)?;
    match coll.insert_one(user.to_doc()?, None) {
      Ok(result) => match result.inserted_id {
        Some(_) => Ok(user),
        None => Err(Error::UserExistedError),
      },
      Err(_) => Err(Error::DatabaseError),
    }
  }

  pub fn remove(conn: &Database, id: &String) -> Result<(), Error> {
    let coll = Self::coll(&conn);
    match coll.delete_one(
      doc! {
        "_id": ObjectId::with_string(id.as_str()).map_err(|_| Error::CannotParseObjectId)?
      },
      None,
    ) {
      Ok(result) => match result.deleted_count {
        1 => Ok(()),
        _ => Err(Error::UserNotFoundError),
      },
      Err(_) => Err(Error::DatabaseError),
    }
  }

  pub fn get_by_oid_and_touch(conn: &Database, id: &ObjectId) -> Result<Self, Error> {
    let coll = Self::coll(&conn);
    match coll.find_one_and_update(
      doc! {
        "_id": id.clone(),
      },
      doc! {
        "$inc": { "visit_count": 1 },
        "$currentDate": { "last_visit_date_time": true },
      },
      None,
    ) {
      Ok(result) => match result {
        Some(doc) => Self::from_doc(doc),
        None => Err(Error::UserNotFoundError),
      },
      Err(_) => Err(Error::DatabaseError),
    }
  }

  pub fn change_password_by_oid(
    conn: &Database,
    id: &ObjectId,
    new_password: &String,
  ) -> Result<(), Error> {
    if Self::is_valid_password(new_password) {
      let coll = Self::coll(&conn);
      match coll.update_one(
        doc! {
          "_id": id.clone()
        },
        doc! {
          "$set": {
            "password": encrypt_password(&new_password),
          },
        },
        None,
      ) {
        Ok(result) => match result.matched_count {
          1 => Ok(()),
          _ => Err(Error::UserNotFoundError),
        },
        Err(_) => Err(Error::DatabaseError),
      }
    } else {
      Err(Error::InvalidPassword)
    }
  }

  pub fn change_password_by_id(
    conn: &Database,
    id: &String,
    new_password: &String,
  ) -> Result<(), Error> {
    let oid = ObjectId::with_string(id.as_str()).map_err(|_| Error::CannotParseObjectId)?;
    Self::change_password_by_oid(conn, &oid, new_password)
  }

  pub fn change_password(&self, conn: &Database, new_password: &String) -> Result<(), Error> {
    Self::change_password_by_oid(conn, self.id(), new_password)
  }

  pub fn setup_collection_index(conn: &Database) -> Result<(), Error> {
    let coll = Self::coll(&conn);
    match coll.create_index(
      doc! {
        "username": 1,
      },
      Some(mongodb::coll::options::IndexOptions {
        unique: Some(true),
        ..Default::default()
      }),
    ) {
      Ok(_) => Ok(()),
      Err(_) => Err(Error::DatabaseError),
    }
  }
}
