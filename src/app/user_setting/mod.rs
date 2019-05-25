use rocket::http::{Cookies};

use crate::util::cookie_value::CookieValue;

mod light_mode;
mod hand_mode;
mod index_display_mode;
mod scale;

pub use light_mode::LightMode;
pub use hand_mode::HandMode;
pub use index_display_mode::IndexDisplayMode;
pub use scale::Scale;

#[derive(Debug, PartialEq)]
pub struct UserSetting {
  pub light_mode: LightMode,
  pub hand_mode: HandMode,
  pub index_display_mode: IndexDisplayMode,
  pub scale: Scale,
}

impl UserSetting {
  pub fn from_cookies(cookies: &Cookies) -> Self {
    Self {
      light_mode: LightMode::from_cookies(cookies),
      hand_mode: HandMode::from_cookies(cookies),
      index_display_mode: IndexDisplayMode::from_cookies(cookies),
      scale: Scale::from_cookies(cookies)
    }
  }
}