use rocket::response::Redirect;

use super::super::AdminUser;
use crate::app::session::Session;
use crate::util::Database;

#[post("/admin/user/session/purge")]
pub fn purge(_admin: AdminUser, conn: Database) -> Redirect {
  println!("[User Session] Purging expired user sessions");
  match Session::purge_expired(&conn) {
    Ok(()) => Redirect::to("/admin/"),
    Err(err) => err.redirect_to_admin(),
  }
}
