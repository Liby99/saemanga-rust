#[derive(Debug, PartialEq, Eq)]
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