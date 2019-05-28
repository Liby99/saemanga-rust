use std::cmp;

use mongodb::oid::ObjectId;
use mongodb::{bson, doc};
use chrono::Utc;

use crate::util::Collection;
use crate::util::Database;
use crate::util::Error;

use super::manga::Manga;
use super::user::User;

#[collection("follow")]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Follow {
  #[serde(rename="_id")]
  id: ObjectId,

  user_id: ObjectId,
  manga_dmk_id: String,

  start_date_time: bson::UtcDateTime,
  update_date_time: bson::UtcDateTime,

  current_episode: i32,
  max_episode: i32,
  is_up_to_date: bool,

  is_liked: bool,
}

impl Follow {
  pub fn new(user: &User, manga: &Manga) -> Result<Self, Error> {
    let now = mongodb::UtcDateTime::from(Utc::now());
    let epi = manga.data().first_episode().episode();
    Ok(Self {
      id: ObjectId::new().map_err(|_| Error::CannotCreateObjectId)?,
      user_id: user.id().clone(),
      manga_dmk_id: manga.data().dmk_id().clone(),
      start_date_time: now,
      update_date_time: now,
      current_episode: epi,
      max_episode: epi,
      is_up_to_date: epi == manga.data().latest_episode().episode(),
      is_liked: false,
    })
  }

  pub fn max_episode(&self) -> i32 {
    self.max_episode
  }

  pub fn get_by_user_and_manga(conn: &Database, user: &User, manga: &Manga) -> Result<Option<Self>, Error> {
    Self::get_one(conn, Some(doc! {
      "user_id": user.id().clone(),
      "manga_dmk_id": manga.data().dmk_id(),
    }), None)
  }

  pub fn get_or_upsert(
    conn: &Database,
    user: &User,
    dmk_id: &String,
    episode: Option<i32>
  ) -> Result<(Self, Manga), Error> {
    let manga = Manga::get_or_fetch_by_dmk_id(conn, dmk_id)?;
    let epi = match episode { Some(e) => e, None => manga.data().first_episode().episode() };
    match Self::get_by_user_and_manga(conn, user, &manga) {
      Ok(maybe_follow) => {
        let follow = match maybe_follow {
          Some(follow) => follow.update(conn, &manga, epi)?,
          None => Self::insert(conn, user, &manga)?,
        };
        Ok((follow, manga))
      },
      Err(err) => Err(err)
    }
  }

  pub fn update(&self, conn: &Database, manga: &Manga, epi: i32) -> Result<Self, Error> {
    let coll = Self::coll(&conn);
    let epi = manga.data().find_episode(epi).ok_or(Error::InvalidEpisode)?.episode();
    match coll.find_one_and_update(doc! {
      "user_id": self.user_id.clone(),
      "manga_dmk_id": self.manga_dmk_id.as_str(),
    }, doc! {
      "$set": {
        "current_episode": epi,
        "max_episode": cmp::max(epi, self.max_episode),
        "is_up_to_date": manga.data().is_latest_episode(epi),
      },
      "$currentDate": {
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

  pub fn insert(conn: &Database, user: &User, manga: &Manga) -> Result<Self, Error> {
    let coll = Self::coll(&conn);
    let follow = Self::new(user, &manga)?;
    match coll.insert_one(follow.to_doc()?, None) {
      Ok(result) => match result.inserted_id {
        Some(_) => Ok(follow),
        None => Err(Error::FollowExistedError)
      },
      Err(_) => Err(Error::DatabaseError)
    }
  }

  pub fn unfollow(conn: &Database, user: &User, dmk_id: &String) -> Result<(), Error> {
    let coll = Self::coll(&conn);
    match coll.delete_one(doc! {
      "user_id": user.id().clone(),
      "manga_dmk_id": dmk_id,
    }, None) {
      Ok(result) => match result.deleted_count {
        1 => Ok(()),
        _ => Err(Error::FollowNotFoundError)
      },
      Err(_) => Err(Error::DatabaseError)
    }
  }

  pub fn update_like(conn: &Database, user: &User, dmk_id: &String, like: bool) -> Result<(), Error> {
    let coll = Self::coll(&conn);
    match coll.update_one(doc! {
      "user_id": user.id().clone(),
      "manga_dmk_id": dmk_id,
    }, doc! {
      "$set": {
        "is_liked": like
      }
    }, None) {
      Ok(result) => match result.matched_count {
        1 => Ok(()),
        _ => Err(Error::FollowNotFoundError)
      },
      Err(err) => {
        println!("{:?}", err);
        Err(Error::DatabaseError)
      }
    }
  }

  pub fn like(conn: &Database, user: &User, dmk_id: &String) -> Result<(), Error> {
    Self::update_like(conn, user, dmk_id, true)
  }

  pub fn unlike(conn: &Database, user: &User, dmk_id: &String) -> Result<(), Error> {
    Self::update_like(conn, user, dmk_id, false)
  }

  pub fn setup_collection_index(conn: &Database) -> Result<(), Error> {
    let coll = Self::coll(&conn);
    match coll.create_index(doc! {
      "user_id": 1,
      "manga_dmk_id": 1,
    }, Some(mongodb::coll::options::IndexOptions {
      unique: Some(true),
      ..Default::default()
    })) {
      Ok(_) => Ok(()),
      Err(_) => Err(Error::DatabaseError),
    }
  }
}