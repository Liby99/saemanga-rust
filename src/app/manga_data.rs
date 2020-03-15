use lazy_static::lazy_static;
use regex::Regex;

use crate::util::Error;
use super::genre::*;

pub fn is_valid_dmk_id(dmk_id: &String) -> bool {
  lazy_static! { static ref DMK_ID_RE : Regex = Regex::new(r"\d{4}").unwrap(); }
  return DMK_ID_RE.is_match(dmk_id);
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MangaStatus {
  #[serde(rename = "ended")] Ended,
  #[serde(rename = "updating")] Updating,
}

impl MangaStatus {
  pub fn ended(&self) -> bool {
    match self {
      MangaStatus::Ended => true,
      _ => false,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MangaEpisode {
  index: i32,
  is_book: bool,
  episode: i32,
  num_pages: i32,
  dmk_directory: String,
}

impl MangaEpisode {
  pub fn new(index: i32, is_book: bool, episode: i32, num_pages: i32, dmk_directory: String) -> MangaEpisode {
    MangaEpisode { index, is_book, episode, num_pages, dmk_directory }
  }

  pub fn index(&self) -> usize {
    self.index as usize
  }

  pub fn is_book(&self) -> bool {
    self.is_book
  }

  pub fn episode(&self) -> i32 {
    self.episode
  }

  pub fn num_pages(&self) -> i32 {
    self.num_pages
  }

  pub fn dmk_directory(&self) -> &String {
    &self.dmk_directory
  }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "version")]
pub enum DmkIdBase {
  V11 { dmk_id_gen: String },
  V10 { dmk_id_web: String, dmk_id_home: String },
  V09 { dmk_id_home: String },
  V08 { dmk_id_home: String },
  V07 { dmk_id_gen: String },
  V06 { dmk_id_gen: String },
  V05 { dmk_id_web: String, dmk_id_gen: String },
}

fn parse_v11_image_url(url: &String) -> Option<DmkIdBase> {
  lazy_static! { static ref IMG_RE_11 : Regex = Regex::new(r"cartoonmad\.com/(\d+)/\d+/\d+/\d+\.jpg").unwrap(); }
  match IMG_RE_11.captures(url.as_str()) {
    Some(cap) => Some(DmkIdBase::V11 { dmk_id_gen: cap[1].to_string() }),
    None => None,
  }
}

fn parse_v10_image_url(url: &String) -> Option<DmkIdBase> {
  lazy_static! { static ref IMG_RE_10 : Regex = Regex::new(r"(web\d*)\.cartoonmad\.com/(home\d+)/(\d+)/\d+/\d+\.jpg").unwrap(); }
  match IMG_RE_10.captures(url.as_str()) {
    Some(cap) => Some(DmkIdBase::V10 { dmk_id_web: cap[1].to_string(), dmk_id_home: cap[2].to_string() }),
    None => None,
  }
}

fn parse_v09_image_url(url: &String) -> Option<DmkIdBase> {
  lazy_static! { static ref IMG_RE_09 : Regex = Regex::new(r"(home\d+)/(\d+)/\d+/\d+\.jpg").unwrap(); }
  match IMG_RE_09.captures(url.as_str()) {
    Some(cap) => Some(DmkIdBase::V09 { dmk_id_home: cap[1].to_string() }),
    None => None,
  }
}

fn parse_v08_image_url(url: &String) -> Option<DmkIdBase> {
  lazy_static! { static ref IMG_RE_08 : Regex = Regex::new(r"^/([\w\d]+)/(\d+)/\d+/\d+\.jpg$").unwrap(); }
  match IMG_RE_08.captures(url.as_str()) {
    Some(cap) => Some(DmkIdBase::V08 { dmk_id_home: cap[1].to_string() }),
    None => None,
  }
}

fn parse_v07_image_url(url: &String) -> Option<DmkIdBase> {
  lazy_static! { static ref IMG_RE_07 : Regex = Regex::new(r"^/home1/([\d\w]+)/(\d+)/\d+/\d+\.jpg$").unwrap(); }
  match IMG_RE_07.captures(url.as_str()) {
    Some(cap) => Some(DmkIdBase::V07 { dmk_id_gen: cap[1].to_string() }),
    None => None,
  }
}

fn parse_v06_image_url(url: &String) -> Option<DmkIdBase> {
  lazy_static! { static ref IMG_RE_06 : Regex = Regex::new(r"^/cartoonimg/([\d\w]+)/(\d+)/\d+/\d+\.jpg$").unwrap(); }
  match IMG_RE_06.captures(url.as_str()) {
    Some(cap) => Some(DmkIdBase::V06 { dmk_id_gen: cap[1].to_string() }),
    None => None,
  }
}

fn parse_v05_image_url(url: &String) -> Option<DmkIdBase> {
  lazy_static! { static ref IMG_RE_05 : Regex = Regex::new(r"^https?://(web\d?)\.cartoonmad\.com/([\w|\d]+)/").unwrap(); }
  match IMG_RE_05.captures(url.as_str()) {
    Some(cap) => Some(DmkIdBase::V05 { dmk_id_web: cap[1].to_string(), dmk_id_gen: cap[2].to_string() }),
    None => None,
  }
}

impl DmkIdBase {
  pub fn from_dmk_image_url(url: &String) -> Result<Self, Error> {
    let functions : Vec<&dyn Fn(&String) -> Option<Self>> = vec![
      &parse_v11_image_url,
      &parse_v10_image_url,
      &parse_v09_image_url,
      &parse_v08_image_url,
      &parse_v07_image_url,
      &parse_v06_image_url,
      &parse_v05_image_url,
    ];
    for f in functions {
      match f(url) {
        Some(res) => return Ok(res),
        _ => ()
      }
    }
    Err(Error::DmkIdBaseParseError)
  }

  pub fn dmk_image_url_base(&self) -> String {
    match self {
      DmkIdBase::V11 { dmk_id_gen } => format!("http://cartoonmad.com/{}", dmk_id_gen),
      DmkIdBase::V10 { dmk_id_web, dmk_id_home } => format!("http://{}.cartoonmad.com/{}", dmk_id_web, dmk_id_home),
      DmkIdBase::V09 { dmk_id_home } => format!("http://cartoonmad.com/{}", dmk_id_home),
      DmkIdBase::V08 { dmk_id_home } => format!("http://cartoonmad.com/{}", dmk_id_home),
      DmkIdBase::V07 { dmk_id_gen } => format!("http://www.cartoonmad.com/home1/{}", dmk_id_gen),
      DmkIdBase::V06 { dmk_id_gen } => format!("http://www.cartoonmad.com/cartoonimg/{}", dmk_id_gen),
      DmkIdBase::V05 { dmk_id_web, dmk_id_gen } => format!("http://{}.cartoonmad.com/{}", dmk_id_web, dmk_id_gen),
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MangaData {
  dmk_id: String,
  dmk_id_base: DmkIdBase,
  title: String,
  description: String,
  author: String,
  tags: Vec<String>,
  genre: &'static Genre,
  status: MangaStatus,
  episodes: Vec<MangaEpisode>,
}

pub fn dmk_cover_url(dmk_id: &String) -> String {
  format!("http://cartoonmad.com/cartoonimg/coimg/{}.jpg", dmk_id)
}

pub fn saemanga_url(dmk_id: &String) -> String {
  format!("/manga/{}", dmk_id)
}

impl MangaData {
  pub fn new(
    dmk_id: String, dmk_id_base: DmkIdBase,
    title: String, description: String, author: String, tags: Vec<String>,
    genre: &'static Genre, status: MangaStatus, episodes: Vec<MangaEpisode>,
  ) -> Self {
    Self { dmk_id, dmk_id_base, title, description, author, tags, genre, status, episodes }
  }

  pub fn dmk_id(&self) -> &String {
    &self.dmk_id
  }

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

  pub fn ended(&self) -> bool {
    self.status == MangaStatus::Ended
  }

  pub fn has_book(&self) -> bool {
    self.episodes.len() > 0 && self.episodes[0].is_book()
  }

  pub fn books(&self) -> Vec<&MangaEpisode> {
    self.episodes.iter().filter(|e| e.is_book()).collect()
  }

  pub fn episodes(&self) -> Vec<&MangaEpisode> {
    self.episodes.iter().filter(|e| !e.is_book()).collect()
  }

  pub fn find_episode(&self, epi: i32) -> Option<&MangaEpisode> {
    self.episodes.iter().find(|&e| e.episode() == epi)
  }

  pub fn next_episode_of(&self, epi: &MangaEpisode) -> Option<&MangaEpisode> {
    assert!(epi.index() < self.episodes.len());
    if epi.index() == self.episodes.len() - 1 {
      None
    } else {
      Some(&self.episodes[epi.index() + 1])
    }
  }

  pub fn prev_episode_of(&self, epi: &MangaEpisode) -> Option<&MangaEpisode> {
    assert!(epi.index() < self.episodes.len());
    if epi.index() == 0 {
      None
    } else {
      Some(&self.episodes[epi.index() - 1])
    }
  }

  pub fn first_episode(&self) -> &MangaEpisode {
    &self.episodes[0]
  }

  pub fn latest_episode(&self) -> &MangaEpisode {
    &self.episodes[self.episodes.len() - 1]
  }

  pub fn is_latest_episode(&self, epi: i32) -> bool {
    self.latest_episode().episode() == epi
  }

  pub fn num_episodes(&self) -> usize {
    self.episodes.len()
  }

  pub fn saemanga_full_url(&self) -> String {
    format!("http://saemanga.com/manga/{}", self.dmk_id)
  }

  pub fn saemanga_url(&self) -> String {
    saemanga_url(&self.dmk_id)
  }

  pub fn saemanga_episode_url(&self, episode: i32) -> String {
    format!("/manga/{}/{}", self.dmk_id, episode)
  }

  pub fn dmk_base_url(&self) -> String {
    format!("https://cartoonmad.com/comic/{}.html", self.dmk_id)
  }

  pub fn dmk_cover_url(&self) -> String {
    dmk_cover_url(&self.dmk_id)
  }

  pub fn dmk_image_url(&self, episode: i32, page: i32) -> String {
    format!("{}/{}/{:03}/{:03}.jpg", self.dmk_id_base.dmk_image_url_base(), self.dmk_id, episode, page)
  }
}