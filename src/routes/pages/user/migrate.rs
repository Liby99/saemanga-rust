use rocket::response::Redirect;
use rocket::request::Form;
use rocket::http::{Cookie, Cookies};

use crate::util::{Error, Database};
use crate::app::temp_password::TEMP_PASSWORD;
use crate::app::user::User;
use crate::app::session::Session;

#[derive(FromForm)]
pub struct MigrateForm {
  new_password: String,
}

#[post("/user/migrate?<redir>", data="<data>")]
pub fn migrate(conn: Database, mut cookies: Cookies, data: Form<MigrateForm>, redir: Option<String>) -> Redirect {
  let redir = match redir { Some(u) => u, None => String::from("/") };
  let err_redir = Error::UserNotQualifiedForMigrate.redirect(Some(redir.as_str()));
  match cookies.get("username") {
    Some(cookie) => {
      let username = cookie.value();
      match User::get_by_username(&conn, &String::from(username)) {
        Ok(user) => {
          if user.is_password_match(&String::from(TEMP_PASSWORD)) {
            match user.change_password(&conn, &data.new_password) {
              Ok(()) => match Session::insert(&conn, &user) {
                Ok(session) => {
                  session.store_to_cookies(&mut cookies);
                  cookies.remove(Cookie::named("username"));
                  Redirect::to(redir)
                },
                Err(err) => err.redirect(Some(redir.as_str()))
              },
              Err(err) => err.redirect(Some(redir.as_str()))
            }
          } else {
            err_redir
          }
        },
        Err(err) => err.redirect(Some(redir.as_str()))
      }
    },
    None => err_redir
  }
}