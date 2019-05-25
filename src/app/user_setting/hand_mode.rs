use rocket::http::{Cookie, Cookies};

use crate::util::cookie_value::CookieValue;
use crate::util::error::Error;

#[derive(Debug, PartialEq, CookieValue)]
pub enum HandMode {
  Right,
  Left,
}