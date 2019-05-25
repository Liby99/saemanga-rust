use rocket::request::Form;
use rocket::response::Redirect;

use crate::util::Database;
use crate::app::user::User;
use super::super::AdminUser;

#[derive(FromForm)]
pub struct CreateUserForm {
  username: String,
  password: String,
}

#[post("/admin/user/create", data="<info>")]
pub fn create(_user: AdminUser, conn: Database, info: Form<CreateUserForm>) -> Redirect {
  match User::insert(&conn, &info.username, &info.password) {
    Ok(_) => Redirect::to("/admin"),
    Err(err) => Redirect::to(format!("/admin/error?code={}", err as u32))
  }
}