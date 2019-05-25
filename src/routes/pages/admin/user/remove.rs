use rocket::request::Form;
use rocket::response::Redirect;

use crate::util::Database;
use crate::app::user::User;
use super::super::AdminUser;

#[derive(FromForm)]
pub struct RemoveUserForm {
  id: String,
}

#[post("/admin/user/remove", data="<info>")]
pub fn remove(_user: AdminUser, conn: Database, info: Form<RemoveUserForm>) -> Redirect {
  match User::remove(&conn, &info.id) {
    Ok(_) => Redirect::to("/admin"),
    Err(err) => Redirect::to(format!("/admin/error?code={}", err as u32))
  }
}