use rocket::http::{Cookie, Cookies};

use crate::util::cookie_value::CookieValue;
use crate::util::error::Error;

#[derive(Debug, PartialEq)]
pub enum IndexDisplayMode {
  All,
  LovedOnly
}

impl CookieValue for IndexDisplayMode {
  type Data = Self;

  const KEY : &'static str = "index-display-mode";

  fn from_str(s: &str) -> Result<Self::Data, Error> {
    match s {
      "all" => Ok(Self::All),
      "loved" => Ok(Self::LovedOnly),
      _ => Err(Error::UnknownIndexDisplayMode),
    }
  }

  fn to_string(&self) -> String {
    match self {
      Self::All => "all".to_string(),
      Self::LovedOnly => "loved".to_string(),
    }
  }

  fn default() -> Self {
    Self::All
  }

  fn from_cookies(cookies: &Cookies) -> Self::Data {
    cookies.get(Self::KEY).map_or(Self::default(), |cookie| {
      Self::from_str(cookie.value()).map_or_else(|_| Self::default(), |v| v)
    })
  }

  fn into_cookies(&self, cookies: &mut Cookies) {
    let value = self.to_string();
    cookies.add(Cookie::build(Self::KEY, value).path("/").finish());
  }
}