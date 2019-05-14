use std::str::FromStr;
use rocket::http::{Cookie, Cookies};

use crate::util::error::Error;

#[derive(Debug, PartialEq)]
pub enum IndexDisplayMode {
  All,
  LovedOnly
}

impl FromStr for IndexDisplayMode {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "left" => Ok(Self::All),
      "right" => Ok(Self::LovedOnly),
      _ => Err(Error::UnknownIndexDisplayMode),
    }
  }
}

impl std::string::ToString for IndexDisplayMode {
  fn to_string(&self) -> String {
    match self {
      Self::All => "left".to_string(),
      Self::LovedOnly => "right".to_string(),
    }
  }
}

impl IndexDisplayMode {
  const KEY : &'static str = "index-display-mode";

  pub fn default() -> Self {
    Self::All
  }

  pub fn from_cookies(cookies: &Cookies) -> Self {
    cookies.get(Self::KEY).map_or(Self::default(), |cookie| {
      Self::from_str(cookie.value()).map_or_else(|_| Self::default(), |v| v)
    })
  }

  pub fn into_cookies(&self, cookies: &mut Cookies) {
    let value = self.to_string();
    cookies.add(Cookie::build(Self::KEY, value).path("/").finish());
  }
}