use rocket::Route;
use rocket::http::Cookies;
use rocket_contrib::json::Json;

use crate::util::cookie_value::CookieValue;

use crate::app::user_setting::*;

pub fn routes() -> Vec<Route> {
  routes![
    set_light_mode,
    set_hand_mode,
  ]
}

#[derive(Serialize, Deserialize)]
struct ChangeSettingResult<'a> {
  success: bool,
  message: &'a str,
}

#[derive(Deserialize)]
struct ChangeModeData {
  mode: String,
}

#[post("/ajax/set_light_mode", data="<data>")]
fn set_light_mode<'a>(mut cookies: Cookies, data: Json<ChangeModeData>) -> Json<ChangeSettingResult<'a>> {
  match LightMode::from_str(data.mode.as_str()) {
    Ok(mode) => {
      mode.into_cookies(&mut cookies);
      Json(ChangeSettingResult { success: true, message: "" })
    },
    Err(err) => Json(ChangeSettingResult { success: false, message: err.msg() })
  }
}

#[post("/ajax/set_hand_mode", data="<data>")]
fn set_hand_mode<'a>(mut cookies: Cookies, data: Json<ChangeModeData>) -> Json<ChangeSettingResult<'a>> {
  match HandMode::from_str(data.mode.as_str()) {
    Ok(mode) => {
      mode.into_cookies(&mut cookies);
      Json(ChangeSettingResult { success: true, message: "" })
    },
    Err(err) => Json(ChangeSettingResult { success: false, message: err.msg() })
  }
}