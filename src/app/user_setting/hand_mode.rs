use std::str::FromStr;
use rocket::http::{Cookie, Cookies};

use crate::util::error::Error;

#[derive(Debug, PartialEq)]
pub enum HandMode {
  Left,
  Right
}

impl FromStr for HandMode {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "left" => Ok(Self::Left),
      "right" => Ok(Self::Right),
      _ => Err(Error::UnknownHandMode),
    }
  }
}

impl std::string::ToString for HandMode {
  fn to_string(&self) -> String {
    match self {
      Self::Left => "left".to_string(),
      Self::Right => "right".to_string(),
    }
  }
}

impl HandMode {
  const KEY : &'static str = "hand-mode";

  pub fn default() -> Self {
    Self::Right
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