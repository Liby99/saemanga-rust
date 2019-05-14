use rocket::response::Redirect;

use crate::app::latest::Latest;
use crate::util::Database;
use super::super::AdminUser;

#[post("/admin/latest/fetch_overall")]
pub fn fetch_overall(_admin: AdminUser, conn: Database) -> Redirect {
  match Latest::fetch_overall(&conn) {
    Ok(_) => Redirect::to("/admin"),
    Err(err) => Redirect::to(format!("/admin/error?code={}", err.code()))
  }
}