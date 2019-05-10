#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaEpisode {
  index: usize,
  is_book: bool,
  episode: u32,
  num_pages: u32,
  dmk_directory: String,
}

impl MangaEpisode {
  pub fn new(index: usize, is_book: bool, episode: u32, num_pages: u32, dmk_directory: String) -> MangaEpisode {
    MangaEpisode { index, is_book, episode, num_pages, dmk_directory }
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

  pub fn dmk_directory(&self) -> &String {
    &self.dmk_directory
  }
}