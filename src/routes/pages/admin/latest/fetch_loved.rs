use rocket::response::Redirect;

use super::super::AdminUser;
use crate::app::manga::Manga;
use crate::util::Database;

#[post("/admin/latest/fetch_loved")]
pub fn fetch_loved(_admin: AdminUser, conn: Database) -> Redirect {
  println!("[Manga Update] Fetching loved (oldest) 50");
  match Manga::fetch_loved(&conn, 50) {
    Ok(_) => Redirect::to("/admin"),
    Err(err) => err.redirect_to_admin(),
  }
}
