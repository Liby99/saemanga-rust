use rocket::Route;
use rocket::http::Cookies;
use rocket_contrib::json::Json;

use std::str::FromStr;
use crate::app::user_setting::*;

pub fn routes() -> Vec<Route> {
  routes![
    set_light_mode
  ]
}

#[derive(Serialize, Deserialize)]
pub struct ChangeSettingResult<'a> {
  success: bool,
  message: &'a str,
}

#[derive(Deserialize)]
pub struct ChangeModeData {
  mode: String,
}

#[post("/ajax/set_light_mode", data="<data>")]
pub fn set_light_mode<'a>(mut cookies: Cookies, data: Json<ChangeModeData>) -> Json<ChangeSettingResult<'a>> {
  match LightMode::from_str(data.mode.as_str()) {
    Ok(mode) => {
      mode.into_cookies(&mut cookies);
      Json(ChangeSettingResult { success: true, message: "" })
    },
    Err(err) => Json(ChangeSettingResult { success: false, message: err.msg() })
  }
}