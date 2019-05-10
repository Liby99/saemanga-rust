use crate::app::user::User;
use crate::util::database::Database;
use rocket::request::Form;
use rocket::response::Redirect;

#[derive(FromForm)]
pub struct CreateUserForm {
  username: String,
  password: String,
}

#[post("/admin/user/create", data="<info>")]
pub fn create(conn: Database, info: Form<CreateUserForm>) -> Redirect {
  match User::insert(&conn, &info.username, &info.password) {
    Ok(_) => Redirect::to("/admin"),
    Err(err) => Redirect::to(format!("/admin/error?code={}", err as u32))
  }
}