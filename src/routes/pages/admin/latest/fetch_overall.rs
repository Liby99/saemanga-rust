use rocket::response::Redirect;

use crate::app::latest_manga::LatestManga;
use crate::util::Database;
use super::super::AdminUser;

#[post("/admin/latest/fetch_overall")]
pub fn fetch_overall(_admin: AdminUser, conn: Database) -> Redirect {
  match LatestManga::fetch_latest(&conn) {
    Ok(_) => Redirect::to("/admin"),
    Err(err) => Redirect::to(format!("/admin/error?code={}", err.code()))
  }
}