#![feature(type_alias_enum_variants)]
#![feature(result_map_or_else)]

use std::string::ToString;
use rocket::http::{Cookie, Cookies};

#[macro_use] extern crate cookie_value;

#[derive(Debug, CookieValue)]
pub enum HandMode {
  Left,
  Right,
}

pub enum Error {
  UnknownHandMode,
}

pub trait CookieValue {
  type Data;

  const KEY : &'static str;

  // FromStr
  fn from_str(s: &str) -> Result<Self::Data, Error>;

  // ToString
  fn to_string(&self) -> String;

  // Cookie values
  fn default() -> Self;
  fn from_cookies(cookies: &Cookies) -> Self::Data;
  fn into_cookies(&self, cookies: &mut Cookies);
}