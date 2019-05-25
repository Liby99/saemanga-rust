use rocket::http::Cookies;

use super::Error;

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
