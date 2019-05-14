use rocket::Route;
use rocket::response::Redirect;
// use rocket::Outcome;
// use rocket::http::Status;
// use rocket::request::{self, Request, FromRequest};

// use crate::util::Database;
// use crate::app::user::User;
// use crate::app::user_session::UserSession;

mod index;
mod login;
mod error;
mod user;
mod manga;

#[get("/admin")]
pub fn root() -> Redirect {
  Redirect::to("/admin/index")
}

pub fn routes() -> Vec<Route> {
  [
    routes![
      root,
      index::index,
      login::login,
      error::error,
    ],
    user::routes(),
    manga::routes(),
  ].concat()
}