use rocket::response::Redirect;
use rocket::http::{Cookies};
use rocket::http::uri::Uri;

use crate::util::{Error, Database};
use crate::app::session::Session;

#[get("/user/logout?<redir>")]
pub fn logout(conn: Database, mut cookies: Cookies, redir: Option<String>) -> Redirect {
  match Session::remove_from_cookies(&conn, &mut cookies) {
    Ok(()) => {
      let redir = match redir {
        Some(url) => match Uri::percent_decode(url.as_bytes()) {
          Ok(decoded) => String::from(decoded),
          Err(_) => format!("/error?code={}", Error::DecodeUrlError.code())
        },
        None => String::from("/")
      };
      println!("Redirect to... {}", redir);
      Redirect::to(redir)
    },
    Err(err) => match redir {
      Some(url) => Redirect::to(format!("/error?code={}&redir={}", err.code(), url)),
      None => Redirect::to(format!("/error?code={}", err.code())),
    }
  }
}