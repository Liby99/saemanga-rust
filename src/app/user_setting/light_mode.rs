use std::str::FromStr;
use rocket::http::{Cookie, Cookies};

use crate::util::error::Error;

#[derive(Debug, PartialEq)]
pub enum LightMode {
  Day,
  Night
}

impl FromStr for LightMode {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "day" => Ok(Self::Day),
      "night" => Ok(Self::Night),
      _ => Err(Error::UnknownLightMode),
    }
  }
}

impl std::string::ToString for LightMode {
  fn to_string(&self) -> String {
    match self {
      Self::Day => "day".to_string(),
      Self::Night => "night".to_string(),
    }
  }
}

impl LightMode {
  const KEY : &'static str = "light-mode";

  pub fn default() -> Self {
    Self::Day
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