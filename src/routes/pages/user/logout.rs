use rocket::http::Cookies;
use rocket::response::Redirect;

use crate::app::session::Session;
use crate::util::Database;

#[get("/user/logout?<redir>")]
pub fn logout(conn: Database, mut cookies: Cookies, redir: Option<String>) -> Redirect {
  match Session::remove_from_cookies(&conn, &mut cookies) {
    Ok(()) => Redirect::to(match redir {
      Some(url) => url,
      None => String::from("/"),
    }),
    Err(err) => match redir {
      Some(url) => err.redirect(Some(url.as_str())),
      None => err.redirect(None),
    },
  }
}
