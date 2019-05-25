use rocket::http::{Cookies};

use crate::util::cookie_value::CookieValue;

mod light_mode;
mod hand_mode;
mod index_display_mode;

pub use light_mode::LightMode;
pub use hand_mode::HandMode;
pub use index_display_mode::IndexDisplayMode;

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
      hand_mode: HandMode::from_cookies(cookies),
      index_display_mode: IndexDisplayMode::from_cookies(cookies),
      scale: extract_scale(cookies)
    }
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