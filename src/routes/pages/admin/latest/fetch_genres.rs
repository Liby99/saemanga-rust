use rocket::response::Redirect;

use crate::app::latest::Latest;
use crate::util::Database;
use super::super::AdminUser;

#[post("/admin/latest/fetch_genres")]
pub fn fetch_genres(_admin: AdminUser, conn: Database) -> Redirect {
  match Latest::fetch_all_genres(&conn) {
    Ok(_) => Redirect::to("/admin"),
    Err(err) => Redirect::to(format!("/admin/error?code={}", err.code()))
  }
}