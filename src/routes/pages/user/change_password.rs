use rocket::response::Redirect;
use rocket::request::Form;

use crate::util::{Error, Database};
use crate::app::user::User;

#[derive(FromForm)]
pub struct ChangePasswordForm {
  old_password: String,
  new_password: String,
}

#[post("/user/change_password?<redir>", data="<data>")]
pub fn change_password(conn: Database, user: &User, data: Form<ChangePasswordForm>, redir: Option<String>) -> Redirect {
  let redir = match redir { Some(u) => u, None => String::from("/") };
  match user.is_password_match(&data.old_password) {
    true => match user.change_password(&conn, &data.new_password) {
      Ok(()) => Redirect::to(redir),
      Err(err) => err.redirect(Some(redir.as_str()))
    },
    false => Error::IncorrectOldPassword.redirect(Some(redir.as_str()))
  }
}