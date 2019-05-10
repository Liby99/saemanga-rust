use lazy_static::lazy_static;
use regex::Regex;

pub use super::dmk_id_base::*;
pub use super::manga_status::*;
pub use super::manga_episode::*;

use super::genre::*;

pub fn is_valid_dmk_id(dmk_id: &String) -> bool {
  lazy_static! { static ref DMK_ID_RE : Regex = Regex::new(r"\d{4}").unwrap(); }
  return DMK_ID_RE.is_match(dmk_id);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Manga {
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

impl Manga {
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

  pub fn has_book(&self) -> bool {
    self.episodes.len() > 0 && self.episodes[0].is_book()
  }

  pub fn books(&self) -> Vec<&MangaEpisode> {
    self.episodes.iter().filter(|e| e.is_book()).collect()
  }

  pub fn episodes(&self) -> Vec<&MangaEpisode> {
    self.episodes.iter().filter(|e| !e.is_book()).collect()
  }

  pub fn find_episode(&self, epi: u32) -> Option<&MangaEpisode> {
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

  pub fn num_episodes(&self) -> usize {
    self.episodes.len()
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
    format!("http://cartoonmad.com/cartoonimgs/coimg/{}.jpg", self.dmk_id)
  }

  pub fn dmk_image_url(&self, episode: u32, page: u32) -> String {
    format!("{}/{}/{:03}/{:03}.jpg", self.dmk_id_base.dmk_image_url_base(), self.dmk_id, episode, page)
  }
}