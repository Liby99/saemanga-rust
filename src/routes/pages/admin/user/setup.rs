use rocket::response::Redirect;

use crate::util::database::Database;
use crate::app::user::User;

#[post("/admin/user/setup")]
pub fn setup(conn: Database) -> Redirect {
  match User::setup_collection_index(&conn) {
    Ok(_) => Redirect::to("/admin/index"),
    Err(err) => Redirect::to(format!("/admin/error?code={}", err as u32))
  }
}