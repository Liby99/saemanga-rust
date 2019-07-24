use rocket::response::Redirect;

use crate::app::manga::Manga;
use crate::util::Database;
use super::super::AdminUser;

#[post("/admin/latest/fetch_genres")]
pub fn fetch_genres(_admin: AdminUser, conn: Database) -> Redirect {
  println!("[Manga Update] Fetching all genre updates");
  match Manga::fetch_all_genres(&conn) {
    Ok(_) => Redirect::to("/admin"),
    Err(err) => err.redirect_to_admin()
  }
}