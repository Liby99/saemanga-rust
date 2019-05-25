use rocket::http::{Cookie, Cookies};

use crate::util::cookie_value::CookieValue;
use crate::util::error::Error;

#[derive(Debug, PartialEq)]
pub struct Scale(f32);

impl Scale {
  pub fn get(&self) -> f32 {
    self.0
  }
}

impl CookieValue for Scale {
  type Data = Self;

  const KEY : &'static str = "scale";

  fn from_str(s: &str) -> Result<Self::Data, Error> {
    match s.parse::<f32>() {
      Ok(v) => Ok(Scale(v)),
      _ => Err(Error::UnknownHandMode),
    }
  }

  fn to_string(&self) -> String {
    self.0.to_string()
  }

  fn default() -> Self {
    Scale(1.0)
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