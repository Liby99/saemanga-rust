use super::user_session::UserSession;
use crate::util::database::Database;
use lazy_static::lazy_static;
use regex::Regex;
use mongodb::oid::ObjectId;
use mongodb::ordered::OrderedDocument;
use mongodb::{Bson, bson, doc};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use chrono::Utc;

enum_from_primitive! {
  #[derive(Debug)]
  pub enum UserError {
    DatabaseError = 1000, // We start the user error from 1000
    UserIdError,
    UserNotFoundError,
    UserDataError,
    UserExistedError,
    InvalidUsername,
    InvalidPassword,
  }
}

impl From<mongodb::oid::Error> for UserError {
  fn from(_: mongodb::oid::Error) -> Self {
    UserError::UserIdError
  }
}

impl From<mongodb::Error> for UserError {
  fn from(_: mongodb::Error) -> Self {
    UserError::DatabaseError
  }
}

#[derive(Serialize, Deserialize)]
pub struct User {
  #[serde(rename="_id")]
  id: ObjectId,
  username: String,
  password: String,
  is_admin: bool,
  visit_count: i32,
  register_date_time: bson::UtcDateTime,
  last_visit_date_time: bson::UtcDateTime,
  sessions: Vec<UserSession>,
}

impl User {
  pub fn coll(conn: &Database) -> mongodb::coll::Collection {
    conn.collection("user")
  }

  pub fn setup_collection_index(conn: &Database) -> Result<(), UserError> {
    let coll = Self::coll(&conn);
    match coll.create_index(doc! {
      "username": 1
    }, Some(mongodb::coll::options::IndexOptions {
      unique: Some(true),
      ..Default::default()
    })) {
      Ok(_) => Ok(()),
      Err(err) => Err(UserError::from(err)),
    }
  }

  pub fn from_bson(bs: Bson) -> Result<Self, UserError> {
    match bson::from_bson::<User>(bs) {
      Ok(user) => Ok(user),
      Err(_) => Err(UserError::UserDataError)
    }
  }

  pub fn from_doc(doc: OrderedDocument) -> Result<Self, UserError> {
    Self::from_bson(bson::Bson::Document(doc))
  }

  pub fn to_bson(&self) -> Result<Bson, UserError> {
    match bson::to_bson(&self) {
      Ok(bs) => Ok(bs),
      Err(_) => {
        println!("BSON...");
        Err(UserError::UserDataError)
      }
    }
  }

  pub fn to_doc(&self) -> Result<OrderedDocument, UserError> {
    self.to_bson().and_then(|bs| match bs {
      Bson::Document(doc) => Ok(doc),
      _ => {
        println!("Doc...");
        Err(UserError::UserDataError)
      },
    })
  }

  pub fn get_all(conn: &Database) -> Result<Vec<Self>, UserError> {
    let coll = Self::coll(&conn);
    let cursor = coll.find(None, None)?;
    let users = cursor.map(|result| match result {
      Ok(doc) => Self::from_doc(doc),
      Err(_) => Err(UserError::DatabaseError)
    }).filter_map(Result::ok).collect::<Vec<_>>();
    Ok(users)
  }

  pub fn get_one(conn: &Database, doc: OrderedDocument) -> Result<Self, UserError> {
    let coll = Self::coll(&conn);
    let option_user_doc = coll.find_one(Some(doc), None)?;
    match option_user_doc {
      Some(user_doc) => Self::from_doc(user_doc),
      None => Err(UserError::UserNotFoundError)
    }
  }

  pub fn get_by_id(conn: &Database, id: &String) -> Result<Self, UserError> {
    Self::get_one(&conn, doc! { "_id": ObjectId::with_string(id.as_str())? })
  }

  pub fn get_by_username(conn: &Database, username: &String) -> Result<Self, UserError> {
    Self::get_one(&conn, doc! { "username": username.to_lowercase() })
  }

  pub fn encrypt(password: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(&password.as_str());
    hasher.result_str()
  }

  pub fn is_valid_username(username: &String) -> bool {
    lazy_static!{ static ref USERNAME_REG : Regex = Regex::new(r"^[A-Za-z0-9\-_\[\]]{4,16}$").unwrap(); }
    USERNAME_REG.is_match(&username.as_str())
  }

  pub fn is_valid_password(password: &String) -> bool {
    lazy_static!{ static ref PASSWORD_REG : Regex = Regex::new(r"^[A-Za-z0-9\-_\.#*@]{8,32}$").unwrap(); }
    PASSWORD_REG.is_match(&password.as_str())
  }

  pub fn new(username: &String, password: &String) -> Result<Self, UserError> {
    if Self::is_valid_password(&password) {
      if Self::is_valid_username(&username) {
        let hashed_pwd = Self::encrypt(&password);
        let now = mongodb::UtcDateTime::from(Utc::now());
        Ok(User {
          id: ObjectId::new()?,
          username: username.to_lowercase(), // Always use lower case to store username
          password: hashed_pwd,
          is_admin: false,
          visit_count: 0,
          register_date_time: now,
          last_visit_date_time: now,
          sessions: vec![]
        })
      } else {
        Err(UserError::InvalidUsername)
      }
    } else {
      Err(UserError::InvalidPassword)
    }
  }

  pub fn insert(conn: &Database, username: &String, password: &String) -> Result<Self, UserError> {
    let coll = Self::coll(&conn);
    let user = Self::new(&username, &password)?;
    match coll.insert_one(user.to_doc()?, None) {
      Ok(result) => match result.inserted_id {
        Some(_) => Ok(user),
        None => Err(UserError::UserExistedError)
      },
      Err(_) => Err(UserError::DatabaseError)
    }
  }

  pub fn remove(conn: &Database, id: &String) -> Result<(), UserError> {
    let coll = Self::coll(&conn);
    match coll.delete_one(doc! { "_id": ObjectId::with_string(id.as_str())? }, None) {
      Ok(result) => match result.deleted_count {
        1 => Ok(()),
        _ => Err(UserError::UserNotFoundError)
      },
      Err(_) => Err(UserError::DatabaseError)
    }
  }

  pub fn change_password(conn: &Database, id: &String, new_password: &String) -> Result<(), UserError> {
    if Self::is_valid_password(new_password) {
      let coll = Self::coll(&conn);
      match coll.update_one(doc! {
        "_id": ObjectId::with_string(id.as_str())?
      }, doc! {
        "$set": {
          "password": Self::encrypt(&new_password)
        }
      }, None) {
        Ok(result) => match result.modified_count {
          1 => Ok(()),
          _ => Err(UserError::UserNotFoundError)
        },
        Err(_) => Err(UserError::DatabaseError)
      }
    } else {
      Err(UserError::InvalidPassword)
    }
  }

  pub fn is_password_match(conn: &Database, id: &String, password: &String) -> Result<bool, UserError> {
    match Self::get_by_id(conn, id) {
      Ok(user) => {
        let hashed_pwd = Self::encrypt(password);
        Ok(hashed_pwd == user.password)
      },
      Err(err) => Err(err)
    }
  }

  pub fn id(&self) -> &ObjectId {
    &self.id
  }

  pub fn username(&self) -> &String {
    &self.username
  }

  pub fn register_date_time(&self) -> &bson::UtcDateTime {
    &self.register_date_time
  }

  pub fn visit_count(&self) -> i32 {
    self.visit_count
  }
}