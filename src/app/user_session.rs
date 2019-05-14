use mongodb::oid::ObjectId;
use mongodb::{bson, doc};
use rocket::http::{Cookie, Cookies};
use chrono::Utc;
use time::{self, Duration};

use super::user::User;
use crate::util::Collection;
use crate::util::Database;
use crate::util::Error;

#[collection("session")]
#[derive(Serialize, Deserialize)]
pub struct UserSession {
  #[serde(rename="_id")]
  session_id: mongodb::oid::ObjectId,
  user_id: mongodb::oid::ObjectId,
  start_date_time: bson::UtcDateTime,
  last_login_date_time: bson::UtcDateTime,
  expire_date_time: bson::UtcDateTime,
}

impl UserSession {
  pub fn key<'a>() -> &'a str {
    "session"
  }

  pub fn new(user: &User) -> Result<UserSession, Error> {
    let now = mongodb::UtcDateTime::from(Utc::now());
    let expire = mongodb::UtcDateTime::from(Utc::now() + Duration::days(30));
    Ok(UserSession {
      session_id: ObjectId::new().map_err(|_| Error::CannotCreateObjectId)?,
      user_id: user.id().clone(),
      start_date_time: now,
      last_login_date_time: now,
      expire_date_time: expire,
    })
  }

  pub fn from_cookies(conn: &Database, cookies: &mut Cookies) -> Result<UserSession, Error> {
    let cookie = cookies.get(Self::key()).ok_or(Error::NoSession)?;
    let session_id = cookie.value().to_string();
    let session = Self::get_by_session_id_and_touch(&conn, &session_id)?;
    session.store_to_cookies(cookies);
    Ok(session)
  }

  pub fn store_to_cookies(&self, cookies: &mut Cookies) {
    let dur = Duration::days(30);
    let expire = time::now() + dur;
    let builder = Cookie::build(Self::key(), self.session_id());
    let cookie = builder.expires(expire).max_age(dur).secure(false).path("/").finish();
    cookies.add(cookie);
  }

  pub fn session_id(&self) -> String {
    self.session_id.to_hex()
  }

  pub fn user_id(&self) -> String {
    self.user_id.to_hex()
  }

  pub fn user(&self, conn: &Database) -> Result<User, Error> {
    User::get_by_oid(conn, &self.user_id)
  }

  pub fn expired(&self) -> bool {
    let now = Utc::now();
    self.expire_date_time < mongodb::UtcDateTime::from(now)
  }

  pub fn get_by_session_id(conn: &Database, session_id: &String) -> Result<Self, Error> {
    Self::get_one(conn, Some(doc! {
      "_id": ObjectId::with_string(session_id.as_str()).map_err(|_| Error::CannotParseObjectId)?
    }), None).and_then(|res| {
      res.ok_or(Error::SessionNotFound).and_then(|session| {
        if session.expired() { Err(Error::SessionExpired) } else { Ok(session) }
      })
    })
  }

  pub fn get_by_session_id_and_touch(conn: &Database, session_id: &String) -> Result<Self, Error> {
    let coll = Self::coll(conn);
    let expire = bson::UtcDateTime::from(Utc::now() + Duration::days(30));
    match coll.find_one_and_update(doc! {
      "_id": ObjectId::with_string(session_id.as_str()).map_err(|_| Error::CannotParseObjectId)?
    }, doc! {
      "$currentDate": { "last_login_date_time": true },
      "$set": { "expire_date_time": bson::to_bson(&expire).map_err(|_| Error::SerializeError)? }
    }, None) {
      Ok(result) => match result {
        Some(doc) => {
          let session = Self::from_doc(doc)?;
          if session.expired() { Err(Error::SessionExpired) } else { Ok(session) }
        },
        None => Err(Error::SessionNotFound),
      },
      Err(_) => Err(Error::DatabaseError),
    }
  }

  pub fn insert(conn: &Database, user: &User) -> Result<UserSession, Error> {
    let coll = Self::coll(&conn);
    let session = Self::new(user)?;
    match coll.insert_one(session.to_doc()?, None) {
      Ok(_) => Ok(session),
      Err(_) => Err(Error::DatabaseError)
    }
  }

  pub fn remove(conn: &Database, session_id: &String) -> Result<(), Error> {
    let coll = Self::coll(&conn);
    match coll.delete_one(doc! {
      "_id": ObjectId::with_string(session_id.as_str()).map_err(|_| Error::CannotParseObjectId)?
    }, None) {
      Ok(result) => match result.deleted_count {
        1 => Ok(()),
        _ => Err(Error::SessionNotFound)
      },
      Err(_) => Err(Error::DatabaseError)
    }
  }
}