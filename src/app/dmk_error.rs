use std::option::NoneError;

#[derive(Debug)]
pub struct DmkError(String);

impl DmkError {
  pub fn new(msg: String) -> Self {
    DmkError(msg)
  }
}

impl std::fmt::Display for DmkError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Scraper Error: {}", self.0)
  }
}

impl From<NoneError> for DmkError {
  fn from(error: NoneError) -> Self {
    Self::new(format!("Unwrapping none error: {:?}", error))
  }
}

impl From<reqwest::Error> for DmkError {
  fn from(error: reqwest::Error) -> Self {
    Self::new(format!("Reqwesting webpage error: {:?}", error))
  }
}