use rocket::http::{Cookie, Cookies};

use crate::util::cookie_value::CookieValue;
use crate::util::error::Error;

#[derive(Debug, PartialEq)]
pub enum HandMode {
  Left,
  Right
}

impl CookieValue for HandMode {
  type Data = Self;

  const KEY : &'static str = "hand-mode";

  fn from_str(s: &str) -> Result<Self::Data, Error> {
    match s {
      "left" => Ok(Self::Left),
      "right" => Ok(Self::Right),
      _ => Err(Error::UnknownHandMode),
    }
  }

  fn to_string(&self) -> String {
    match self {
      Self::Left => "left".to_string(),
      Self::Right => "right".to_string(),
    }
  }

  fn default() -> Self {
    Self::Right
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