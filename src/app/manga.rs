use chrono::prelude::*;
use mongodb::oid::ObjectId;

use super::genre::*;

pub enum MangaStatus {
  Ended,
  Updating,
}

impl MangaStatus {
  pub fn ended(&self) -> bool {
    match self {
      MangaStatus::Ended => true,
      _ => false,
    }
  }
}

pub struct MangaEpisode {
  index: u32,
  is_book: bool,
  episode: u32,
  num_pages: u32,
}

impl MangaEpisode {
  pub fn new(index: u32, is_book: bool, episode: u32, num_pages: u32) -> MangaEpisode {
    MangaEpisode { 
      index: index,
      is_book: is_book,
      episode: episode, 
      num_pages: num_pages,
    }
  }

  pub fn index(&self) -> u32 {
    self.index
  }

  pub fn is_book(&self) -> bool {
    self.is_book
  }

  pub fn episode(&self) -> u32 {
    self.episode
  }

  pub fn num_pages(&self) -> u32 {
    self.num_pages
  }
}

pub enum MangaDmkIdBase {
  V10 { dmk_id_web: String, dmk_id_home: String },
  V09 { dmk_id_home: String },
  V08 { dmk_id_home: String },
  V07 { dmk_id_gen: String },
  V06 { dmk_id_gen: String },
  V05 { dmk_id_web: String, dmk_id_gen: String },
}

impl MangaDmkIdBase {
  pub fn dmk_image_url_base(&self) -> String {
    match self {
      MangaDmkIdBase::V10 { dmk_id_web, dmk_id_home } => format!("http://{}.cartoonmad.com/{}", dmk_id_web, dmk_id_home),
      MangaDmkIdBase::V09 { dmk_id_home } => format!("http://cartoonmad.com/{}", dmk_id_home),
      MangaDmkIdBase::V08 { dmk_id_home } => format!("http://cartoonmad.com/{}", dmk_id_home),
      MangaDmkIdBase::V07 { dmk_id_gen } => format!("http://www.cartoonmad.com/home1/{}", dmk_id_gen),
      MangaDmkIdBase::V06 { dmk_id_gen } => format!("http://www.cartoonmad.com/cartoonimg/{}", dmk_id_gen),
      MangaDmkIdBase::V05 { dmk_id_web, dmk_id_gen } => format!("http://{}.cartoonmad.com/{}", dmk_id_web, dmk_id_gen),
    }
  }
}

pub struct Manga {

  // dmk information
  dmk_id: String,
  dmk_id_base: MangaDmkIdBase,

  // Information of the manga
  title: String,
  description: String,
  author: String,
  tags: Vec<String>,
  genre: Genre,
  status: MangaStatus,

  // Episode Information
  episodes: Vec<MangaEpisode>,

  // Other Information
  insert_date: DateTime<Utc>,
  update_date: DateTime<Utc>,
  id: ObjectId,
}

impl Manga {
  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn description(&self) -> &String {
    &self.description
  }

  pub fn author(&self) -> &String {
    &self.author
  }

  pub fn tags(&self) -> &Vec<String> {
    &self.tags
  }

  pub fn genre(&self) -> &Genre {
    &self.genre
  }

  pub fn status(&self) -> &MangaStatus {
    &self.status
  }

  pub fn has_book(&self) -> bool {
    self.episodes.len() > 0 && self.episodes[0].is_book
  }

  pub fn books(&self) -> Vec<&MangaEpisode> {
    self.episodes.iter().filter(|e| e.is_book).collect()
  }

  pub fn episodes(&self) -> Vec<&MangaEpisode> {
    self.episodes.iter().filter(|e| !e.is_book).collect()
  }

  pub fn find_episode(&self, epi: u32) -> Option<&MangaEpisode> {
    self.episodes.iter().find(|&e| e.episode == epi)
  }

  // pub fn next_episode_of(&self, epi: &MangaEpisoe) -> Option<&MangaEpisode>

  // pub fn prev_episode_of(&self, epi: &MangaEpisoe) -> Option<&MangaEpisode>

  pub fn saemanga_url(&self) -> String {
    format!("http://saemanga.com/manga/{}", self.dmk_id)
  }

  pub fn saemanga_episode_url(&self, episode: u32) -> String {
    format!("http://saemanga.com/manga/{}/{}", self.dmk_id, episode)
  }

  pub fn dmk_base_url(&self) -> String {
    format!("https://cartoonmad.com/comic/{}.html", self.dmk_id)
  }

  pub fn dmk_cover_url(&self) -> String {
    format!("http://cartoonmad.com/cartoonimg/coimg/{}.jpg", self.dmk_id)
  }

  pub fn dmk_image_url(&self, episode: u32, page: u32) -> String {
    format!("{}/{}/{:03}/{:03}.jpg", self.dmk_id_base.dmk_image_url_base(), self.dmk_id, episode, page)
  }
}