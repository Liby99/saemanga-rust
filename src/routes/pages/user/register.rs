use rocket::http::Cookies;
use rocket::request::Form;
use rocket::response::Redirect;

use crate::app::session::Session;
use crate::app::user::User;
use crate::util::Database;

#[derive(FromForm)]
pub struct RegisterForm {
  username: String,
  password: String,
}

#[post("/user/register?<redir>", data = "<data>")]
pub fn register(
  conn: Database,
  mut cookies: Cookies,
  data: Form<RegisterForm>,
  redir: Option<String>,
) -> Redirect {
  let redir = match redir {
    Some(u) => u,
    None => String::from("/"),
  };
  match User::insert(&conn, &data.username, &data.password) {
    Ok(user) => match Session::insert(&conn, &user) {
      Ok(session) => {
        session.store_to_cookies(&mut cookies);
        Redirect::to(redir)
      }
      Err(err) => err.redirect(Some(redir.as_str())),
    },
    Err(err) => err.redirect(Some(redir.as_str())),
  }
}
