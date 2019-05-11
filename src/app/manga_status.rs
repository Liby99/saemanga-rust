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