use rocket::response::Redirect;

use crate::util::Database;
use crate::app::session::Session;
use super::super::AdminUser;

#[post("/admin/user/session/purge")]
pub fn purge(_admin: AdminUser, conn: Database) -> Redirect {
  match Session::purge_expired(&conn) {
    Ok(()) => Redirect::to("/admin/"),
    Err(err) => err.redirect_to_admin()
  }
}