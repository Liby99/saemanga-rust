use crate::app::user::{User, UserError};
use crate::util::database::Database;
use rocket::request::Form;
use rocket::response::Redirect;

#[derive(FromForm)]
pub struct CreateUserForm {
  username: String,
  password: String,
}

#[post("/admin/user/create", data="<info>")]
pub fn create_user(conn: Database, info: Form<CreateUserForm>) -> Redirect {
  match User::insert(&conn, &info.username, &info.password) {
    Ok(_) => Redirect::to("/admin"),
    Err(err) => match err {
      UserError::DatabaseError => Redirect::to("/admin/error?code=1000"),
      UserError::UserDataError => Redirect::to("/admin/error?code=1001"),
      UserError::UserIdError => Redirect::to("/admin/error?code=1002"),
      UserError::UserNotFoundError => Redirect::to("/admin/error?code=1003"),
      UserError::UserExistedError => Redirect::to("/admin/error?code=1004"),
      UserError::InvalidPassword => Redirect::to("/admin/error?code=1005"),
      UserError::InvalidUsername => Redirect::to("/admin/error?code=1006"),
    }
  }
}