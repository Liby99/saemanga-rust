use rocket::http::Cookies;

#[derive(Debug)]
pub enum LightMode {
  Day,
  Night
}

#[derive(Debug)]
pub enum HandMode {
  Left,
  Right
}

#[derive(Debug)]
pub enum IndexDisplayMode {
  All,
  LovedOnly
}

#[derive(Debug)]
pub struct UserSetting {
  light_mode: LightMode,
  hand_mode: HandMode,
  index_display_mode: IndexDisplayMode
}

pub fn extract_light_mode(cookies: &Cookies) -> LightMode {
  match cookies.get("light-mode") {
    Some(c) => match c.value() {
      "day" => LightMode::Day,
      "night" => LightMode::Night,
      _ => LightMode::Day
    },
    None => LightMode::Day
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

pub fn extract_user_setting(cookies: &Cookies) -> UserSetting {
  UserSetting {
    light_mode: extract_light_mode(cookies),
    hand_mode: extract_hand_mode(cookies),
    index_display_mode: extract_index_display_mode(cookies)
  }
}