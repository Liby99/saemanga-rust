use rocket::http::{Cookie, Cookies};

use crate::util::cookie_value::CookieValue;
use crate::util::error::Error;

#[derive(Debug, PartialEq)]
pub enum LightMode {
  Day,
  Night
}

impl CookieValue for LightMode {
  type Data = Self;

  const KEY : &'static str = "light-mode";

  fn from_str(s: &str) -> Result<Self::Data, Error> {
    match s {
      "day" => Ok(Self::Day),
      "night" => Ok(Self::Night),
      _ => Err(Error::UnknownIndexDisplayMode),
    }
  }

  fn to_string(&self) -> String {
    match self {
      Self::Day => "day".to_string(),
      Self::Night => "night".to_string(),
    }
  }

  fn default() -> Self {
    Self::Day
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