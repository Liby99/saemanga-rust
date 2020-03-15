use rocket::response::Redirect;

use super::super::AdminUser;
use crate::app::manga::Manga;
use crate::util::Database;

#[post("/admin/latest/fetch_oldest_updating")]
pub fn fetch_oldest_updating(_admin: AdminUser, conn: Database) -> Redirect {
  println!("[Manga Update] Fetching oldest updating 50");
  match Manga::fetch_oldest_updating(&conn, 50) {
    Ok(_) => Redirect::to("/admin"),
    Err(err) => err.redirect_to_admin(),
  }
}
