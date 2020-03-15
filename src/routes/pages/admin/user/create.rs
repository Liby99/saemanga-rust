use rocket::request::Form;
use rocket::response::Redirect;

use super::super::AdminUser;
use crate::app::user::User;
use crate::util::Database;

#[derive(FromForm)]
pub struct CreateUserForm {
  username: String,
  password: String,
}

#[post("/admin/user/create", data = "<info>")]
pub fn create(_user: AdminUser, conn: Database, info: Form<CreateUserForm>) -> Redirect {
  match User::insert(&conn, &info.username, &info.password) {
    Ok(_) => Redirect::to("/admin"),
    Err(err) => err.redirect_to_admin(),
  }
}
