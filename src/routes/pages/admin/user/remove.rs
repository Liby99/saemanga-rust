use rocket::request::Form;
use rocket::response::Redirect;

use super::super::AdminUser;
use crate::app::user::User;
use crate::util::Database;

#[derive(FromForm)]
pub struct RemoveUserForm {
  id: String,
}

#[post("/admin/user/remove", data = "<info>")]
pub fn remove(_user: AdminUser, conn: Database, info: Form<RemoveUserForm>) -> Redirect {
  match User::remove(&conn, &info.id) {
    Ok(_) => Redirect::to("/admin"),
    Err(err) => err.redirect_to_admin(),
  }
}
