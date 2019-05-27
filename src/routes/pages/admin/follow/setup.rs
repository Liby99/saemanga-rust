use rocket::response::Redirect;

use crate::util::Database;
use crate::app::follow::Follow;
use super::super::AdminUser;

#[post("/admin/follow/setup")]
pub fn setup(_user: AdminUser, conn: Database) -> Redirect {
  match Follow::setup_collection_index(&conn) {
    Ok(_) => Redirect::to("/admin/index"),
    Err(err) => err.redirect_to_admin()
  }
}