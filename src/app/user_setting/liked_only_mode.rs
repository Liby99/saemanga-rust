use rocket::http::{Cookie, Cookies};

use crate::util::{CookieValue, Error};

#[derive(Debug, PartialEq, CookieValue)]
pub enum LikedOnlyMode {
  Off,
  On,
}
