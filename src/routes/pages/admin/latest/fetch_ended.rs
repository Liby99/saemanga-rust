use rocket::response::Redirect;

use crate::app::manga::Manga;
use crate::util::Database;
use super::super::AdminUser;

#[post("/admin/latest/fetch_ended")]
pub fn fetch_ended(_admin: AdminUser, conn: Database) -> Redirect {
  println!("[Manga Update] Fetching ended mangas");
  match Manga::fetch_ended(&conn) {
    Ok(_) => Redirect::to("/admin/index"),
    Err(err) => err.redirect_to_admin()
  }
}