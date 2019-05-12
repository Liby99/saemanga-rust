use rocket::response::Redirect;

use crate::util::database::Database;
use crate::app::manga::Manga;

#[post("/admin/manga/setup")]
pub fn setup(conn: Database) -> Redirect {
  match Manga::setup_collection_index(&conn) {
    Ok(_) => Redirect::to("/admin/index"),
    Err(err) => Redirect::to(format!("/admin/error?code={}", err as u32))
  }
}