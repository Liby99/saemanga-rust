use rocket::http::Cookies;
use rocket::Route;
use rocket_contrib::json::Json;

use crate::app::temp_password::TEMP_PASSWORD;
use crate::app::user::User;
use crate::util::Database;

pub fn routes() -> Vec<Route> {
  routes![
    can_migrate_route // just to change the name so no conflict
  ]
}

#[derive(Serialize)]
struct CanMigrateResponse {
  can: bool,
}

#[post("/ajax/user/can_migrate")]
fn can_migrate_route(conn: Database, cookies: Cookies) -> Json<CanMigrateResponse> {
  Json(CanMigrateResponse {
    can: match cookies.get("username") {
      Some(cookie) => {
        let username = cookie.value();
        match User::get_by_username(&conn, &String::from(username)) {
          Ok(user) => user.is_password_match(&String::from(TEMP_PASSWORD)),
          Err(_) => false,
        }
      }
      None => false,
    },
  })
}
