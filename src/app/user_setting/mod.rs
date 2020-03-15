use rocket::http::Cookies;

use crate::util::CookieValue;

mod hand_mode;
mod light_mode;
mod liked_only_mode;
mod scale;

pub use hand_mode::HandMode;
pub use light_mode::LightMode;
pub use liked_only_mode::LikedOnlyMode;
pub use scale::Scale;

#[derive(Debug, PartialEq)]
pub struct UserSetting {
  pub light_mode: LightMode,
  pub hand_mode: HandMode,
  pub liked_only_mode: LikedOnlyMode,
  pub scale: Scale,
}

impl UserSetting {
  pub fn from_cookies(cookies: &Cookies) -> Self {
    Self {
      light_mode: LightMode::from_cookies(cookies),
      hand_mode: HandMode::from_cookies(cookies),
      liked_only_mode: LikedOnlyMode::from_cookies(cookies),
      scale: Scale::from_cookies(cookies),
    }
  }
}
