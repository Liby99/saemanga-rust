use rocket::response::Redirect;

use super::super::AdminUser;
use crate::app::user::User;
use crate::util::Database;

#[post("/admin/user/setup")]
pub fn setup(_user: AdminUser, conn: Database) -> Redirect {
  match User::setup_collection_index(&conn) {
    Ok(_) => Redirect::to("/admin/index"),
    Err(err) => err.redirect_to_admin(),
  }
}
