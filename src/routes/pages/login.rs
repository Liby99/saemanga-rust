use rocket::response::Redirect;
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;

use crate::util::database::Database;
use crate::util::error::Error;
use crate::app::user::User;
use crate::app::user_session::UserSession;

#[derive(FromForm)]
pub struct LoginForm {
  username: String,
  password: String,
  return_url: Option<String>,
}

#[post("/login", data="<data>")]
pub fn login(conn: Database, mut cookies: Cookies, data: Form<LoginForm>) -> Redirect {
  let return_url = match &data.return_url { Some(u) => u.clone(), None => String::from("/") };
  match User::get_by_username(&conn, &data.username) {
    Ok(user) => match user.is_password_match(&data.password) {
      true => match UserSession::insert(&conn, &user) {
        Ok(session) => {
          session.store_to_cookies(&mut cookies);
          Redirect::to(return_url)
        },
        Err(err) => Redirect::to(format!("/admin/error?code={}", err.code()))
      },
      false => Redirect::to(format!("/admin/error?code={}", Error::UsernameOrPasswordError.code())),
    },
    Err(err) => match err {
      Error::UserNotFoundError => Redirect::to(format!("/admin/error?code={}", Error::UsernameOrPasswordError.code())),
      _ => Redirect::to(format!("/admin/error?code={}", err.code()))
    }
  }
}