use rocket::response::Redirect;

use super::super::AdminUser;
use crate::app::follow::Follow;
use crate::util::Database;

#[post("/admin/follow/setup")]
pub fn setup(_user: AdminUser, conn: Database) -> Redirect {
  match Follow::setup_collection_index(&conn) {
    Ok(_) => Redirect::to("/admin/index"),
    Err(err) => err.redirect_to_admin(),
  }
}
