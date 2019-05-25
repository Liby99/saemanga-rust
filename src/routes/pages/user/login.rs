use rocket::response::Redirect;
use rocket::http::{Cookies};
use rocket::request::Form;

use crate::util::{Error, Database};
use crate::app::user::User;
use crate::app::session::Session;

#[derive(FromForm)]
pub struct LoginForm {
  username: String,
  password: String,
}

#[post("/user/login?<redir>", data="<data>")]
pub fn login(conn: Database, mut cookies: Cookies, data: Form<LoginForm>, redir: Option<String>) -> Redirect {
  let redir = match redir { Some(u) => u, None => String::from("/") };
  match User::get_by_username(&conn, &data.username) {
    Ok(user) => match user.is_password_match(&data.password) {
      true => match Session::insert(&conn, &user) {
        Ok(session) => {
          session.store_to_cookies(&mut cookies);
          Redirect::to(redir)
        },
        Err(err) => err.redirect(Some(redir.as_str()))
      },
      false => Error::UsernameOrPasswordError.redirect(Some(redir.as_str())),
    },
    Err(err) => match err {
      Error::UserNotFoundError => Error::UsernameOrPasswordError.redirect(Some(redir.as_str())),
      _ => err.redirect(Some(redir.as_str()))
    }
  }
}