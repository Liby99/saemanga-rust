use rocket::response::Redirect;

use crate::app::manga::Manga;
use crate::util::Database;
use super::super::AdminUser;

#[post("/admin/latest/fetch_oldest_updating")]
pub fn fetch_oldest_updating(_admin: AdminUser, conn: Database) -> Redirect {
  match Manga::fetch_oldest_updating(&conn, 50) {
    Ok(_) => Redirect::to("/admin"),
    Err(err) => err.redirect_to_admin()
  }
}