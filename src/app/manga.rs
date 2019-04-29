use lazy_static::lazy_static;
use chrono::prelude::*;
use mongodb::oid::ObjectId;
use regex::Regex;

use super::genre::*;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct MangaEpisode {
  index: usize,
  is_book: bool,
  episode: u32,
  num_pages: u32,
}

impl MangaEpisode {
  pub fn new(index: usize, is_book: bool, episode: u32, num_pages: u32) -> MangaEpisode {
    MangaEpisode { 
      index: index,
      is_book: is_book,
      episode: episode, 
      num_pages: num_pages,
    }
  }

  pub fn index(&self) -> usize {
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

#[derive(Debug, PartialEq)]
pub enum MangaDmkIdBase {
  V10 { dmk_id_web: String, dmk_id_home: String },
  V09 { dmk_id_home: String },
  V08 { dmk_id_home: String },
  V07 { dmk_id_gen: String },
  V06 { dmk_id_gen: String },
  V05 { dmk_id_web: String, dmk_id_gen: String },
}

fn parse_v10_image_url(url: &String) -> Option<MangaDmkIdBase> {
  lazy_static! { static ref IMG_RE_10 : Regex = Regex::new(r"(web\d+)\.cartoonmad\.com/(home\d+)/(\d+)/\d+/\d+\.jpg").unwrap(); }
  match IMG_RE_10.captures(url.as_str()) {
    Some(cap) => Some(MangaDmkIdBase::V10 { dmk_id_web: cap[1].to_string(), dmk_id_home: cap[2].to_string() }),
    None => None
  }
}

fn parse_v09_image_url(url: &String) -> Option<MangaDmkIdBase> {
  lazy_static! { static ref IMG_RE_09 : Regex = Regex::new(r"(home\d+)/(\d+)/\d+/\d+\.jpg").unwrap(); }
  match IMG_RE_09.captures(url.as_str()) {
    Some(cap) => Some(MangaDmkIdBase::V09 { dmk_id_home: cap[1].to_string() }),
    None => None
  }
}

fn parse_v08_image_url(url: &String) -> Option<MangaDmkIdBase> {
  lazy_static! { static ref IMG_RE_08 : Regex = Regex::new(r"^/([\w\d]+)/(\d+)/\d+/\d+\.jpg$").unwrap(); }
  match IMG_RE_08.captures(url.as_str()) {
    Some(cap) => Some(MangaDmkIdBase::V08 { dmk_id_home: cap[1].to_string() }),
    None => None
  }
}

fn parse_v07_image_url(url: &String) -> Option<MangaDmkIdBase> {
  lazy_static! { static ref IMG_RE_07 : Regex = Regex::new(r"^/home1/([\d\w]+)/(\d+)/\d+/\d+\.jpg$").unwrap(); }
  match IMG_RE_07.captures(url.as_str()) {
    Some(cap) => Some(MangaDmkIdBase::V07 { dmk_id_gen: cap[1].to_string() }),
    None => None
  }
}

fn parse_v06_image_url(url: &String) -> Option<MangaDmkIdBase> {
  lazy_static! { static ref IMG_RE_06 : Regex = Regex::new(r"^/cartoonimg/([\d\w]+)/(\d+)/\d+/\d+\.jpg$").unwrap(); }
  match IMG_RE_06.captures(url.as_str()) {
    Some(cap) => Some(MangaDmkIdBase::V06 { dmk_id_gen: cap[1].to_string() }),
    None => None
  }
}

fn parse_v05_image_url(url: &String) -> Option<MangaDmkIdBase> {
  lazy_static! { static ref IMG_RE_05 : Regex = Regex::new(r"^https?://(web\d?)\.cartoonmad\.com/([\w|\d]+)/").unwrap(); }
  match IMG_RE_05.captures(url.as_str()) {
    Some(cap) => Some(MangaDmkIdBase::V05 { dmk_id_web: cap[1].to_string(), dmk_id_gen: cap[2].to_string() }),
    None => None
  }
}

impl MangaDmkIdBase {
  pub fn from_dmk_image_url(url: &String) -> Option<MangaDmkIdBase> {
    let functions : Vec<&Fn(&String) -> Option<MangaDmkIdBase>> = vec![
      &parse_v10_image_url,
      &parse_v09_image_url,
      &parse_v08_image_url,
      &parse_v07_image_url,
      &parse_v06_image_url,
      &parse_v05_image_url,
    ];
    for f in functions {
      match f(url) {
        Some(res) => {
          return Some(res);
        },
        _ => ()
      }
    }
    None
  }

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

#[derive(Debug)]
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

  pub fn next_episode_of(&self, epi: &MangaEpisode) -> Option<&MangaEpisode> {
    assert!(epi.index < self.episodes.len());
    if epi.index == self.episodes.len() - 1 {
      None
    } else {
      Some(&self.episodes[epi.index + 1])
    }
  }

  pub fn prev_episode_of(&self, epi: &MangaEpisode) -> Option<&MangaEpisode> {
    assert!(epi.index < self.episodes.len());
    if epi.index == 0 {
      None
    } else {
      Some(&self.episodes[epi.index - 1])
    }
  }

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