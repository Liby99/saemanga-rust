use std::Option;
use chrono::prelude::*;
use mongodb::oid::ObjectId;

mod genre;

pub enum MangaStatus {
  Ended,
  Updating,
}

pub struct MangaEpisode {
  episode: u32,
  num_pages: u32,
  update_date: DateTime<Utc>
}

pub struct MangaBook {
  book: u32,
  num_pages: u32,
}

pub struct Manga {

  // dmk information
  dmk_id_ver: u8,
  dmk_id: MangaUrlComponent,
  dmk_id_web: Option<String>,
  dmk_id_home: Option<String>,
  dmk_id_gen: Option<String>,

  // Information of the manga
  title: String,
  description: String,
  author: String,
  tags: Vec<String>,
  genre: Genre,
  status: MangaStatus,

  // Episode Information
  books: Vec<Mangabook>,
  episodes: Vec<MangaEpisode>,

  // Other Information
  insert_date: DateTime<Utc>,
  update_date: DateTime<Utc>,
  id: ObjectId,
}

impl Manga {
  pub fn title(&self) -> &String {
    self.title
  }

  pub fn description(&self) -> &String {
    self.description
  }

  pub fn author(&self) -> &String {
    self.author
  }

  pub fn tags(&self) -> &Vec<String> {
    self.tags
  }

  pub fn genre(&self) -> &Genre {
    self.genre
  }

  pub fn saemanga_base_url(&self) -> String {

  }

  pub fn saemanga_url(&self, episode: u32) -> String {

  }

  pub fn dmk_base_url(&self) -> String {

  }

  pub fn dmk_episode_url(&self, episode: u32) -> String {

  }

  pub fn dmk_image_url(&self, episode: u32, page: u32) -> String {

  }
}

// pub enum MangaUrlComponent {
//   One { dmk_id_web: String, dmk_id_gen: String },
//   Two { dmk_id_gen: String },
//   Three { dmk_id_gen: String },
//   Four { dmk_id_home: String },
//   Five { dmk_id_home: String },
//   Six { dmk_id_web: String, dmk_id_home: String },
// }