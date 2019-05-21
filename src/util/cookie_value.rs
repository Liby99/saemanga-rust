// use std::str::FromStr;
// use std::string::ToString;
// use rocket::http::Cookies;

// use super::error::Error;

// pub trait CookieValue: FromStr + ToString {
//   const KEY : &'static str;

//   // FromStr
//   fn from_str(s: &str) -> Result<Self, Error>;

//   // ToString
//   fn to_string(&self) -> String;

//   // Cookie values
//   fn default() -> Self;
//   fn from_cookies(cookies: &Cookies) -> Self;
//   fn into_cookies(&self, cookies: &mut Cookies);
// }
