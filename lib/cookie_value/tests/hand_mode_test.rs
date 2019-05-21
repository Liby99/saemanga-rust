use std::str::FromStr;
use std::string::ToString;
use rocket::http::{Cookie, Cookies};

#[macro_use] extern crate cookie_value;

enum Error {
  UnknownHandMode,   
}

pub trait CookieValue: FromStr + ToString {
  const KEY : &'static str;

  // FromStr
  fn from_str(s: &str) -> Result<Self, Self::Err>;

  // ToString
  fn to_string(&self) -> String;

  // Cookie values
  fn default() -> Self;
  fn from_cookies(cookies: &Cookies) -> Self;
  fn into_cookies(&self, cookies: &mut Cookies);
}

impl CookieValue for HandMode {
  type Err = Error;

  const KEY : &'static str = "hand_mode";

  fn from_str(s: &str) -> Result<Self, FromStr::Err> {
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

  fn from_cookies(cookies: &Cookies) -> Self {
    cookies.get(Self::KEY).map_or(Self::default(), |cookie| {
      Self::from_str(cookie.value()).map_or_else(|_| Self::default(), |v| v)
    })
  }

  fn into_cookies(&self, cookies: &mut Cookies) {
    let value = self.to_string();
    cookies.add(Cookie::build(Self::KEY, value).path("/").finish());
  }
}


#[derive(Debug)]
enum HandMode {
  Left,
  Right,
}
