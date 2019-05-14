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

#[derive(Debug, PartialEq)]
pub enum HandMode {
  Left,
  Right
}

#[derive(Debug, PartialEq)]
pub enum IndexDisplayMode {
  All,
  LovedOnly
}

#[derive(Debug, PartialEq)]
pub struct UserSetting {
  pub light_mode: LightMode,
  pub hand_mode: HandMode,
  pub index_display_mode: IndexDisplayMode,
  pub scale: f32,
}

impl UserSetting {
  pub fn from_cookies(cookies: &Cookies) -> Self {
    Self {
      light_mode: LightMode::from_cookies(cookies),
      hand_mode: extract_hand_mode(cookies),
      index_display_mode: extract_index_display_mode(cookies),
      scale: extract_scale(cookies)
    }
  }
}

pub fn extract_hand_mode(cookies: &Cookies) -> HandMode {
  match cookies.get("hand-mode") {
    Some(c) => match c.value() {
      "left" => HandMode::Left,
      "right" => HandMode::Right,
      _ => HandMode::Right
    },
    None => HandMode::Right
  }
}

pub fn extract_index_display_mode(cookies: &Cookies) -> IndexDisplayMode {
  match cookies.get("index-display-mode") {
    Some(c) => match c.value() {
      "all" => IndexDisplayMode::All,
      "loved" => IndexDisplayMode::LovedOnly,
      _ => IndexDisplayMode::All
    },
    None => IndexDisplayMode::All
  }
}

pub fn extract_scale(cookies: &Cookies) -> f32 {
  match cookies.get("scale") {
    Some(c) => match c.value().parse::<f32>() {
      Ok(v) => v,
      _ => 1.0
    },
    None => 1.0
  }
}