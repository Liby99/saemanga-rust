use rocket::response::Redirect;

use crate::util::database::Database;
use crate::app::manga::Manga;
use super::super::AdminUser;

#[post("/admin/manga/setup")]
pub fn setup(_user: AdminUser, conn: Database) -> Redirect {
  match Manga::setup_collection_index(&conn) {
    Ok(_) => Redirect::to("/admin/index"),
    Err(err) => Redirect::to(format!("/admin/error?code={}", err as u32))
  }
}