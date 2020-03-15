use rocket::response::Redirect;

use super::super::AdminUser;
use crate::app::manga::Manga;
use crate::util::Database;

#[post("/admin/manga/setup")]
pub fn setup(_user: AdminUser, conn: Database) -> Redirect {
  match Manga::setup_collection_index(&conn) {
    Ok(_) => Redirect::to("/admin/index"),
    Err(err) => err.redirect_to_admin(),
  }
}
