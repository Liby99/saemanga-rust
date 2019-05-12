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