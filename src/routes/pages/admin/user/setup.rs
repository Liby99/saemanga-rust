use rocket::response::Redirect;

use crate::util::Database;
use crate::app::user::User;
use super::super::AdminUser;

#[post("/admin/user/setup")]
pub fn setup(_user: AdminUser, conn: Database) -> Redirect {
  match User::setup_collection_index(&conn) {
    Ok(_) => Redirect::to("/admin/index"),
    Err(err) => Redirect::to(format!("/admin/error?code={}", err as u32))
  }
}