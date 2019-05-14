use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use rocket::{Route, Catcher};

use crate::util::Database;
use crate::util::Error;
use crate::app::user::User;
use crate::app::user_session::UserSession;

mod index;
mod login;
mod logout;
mod error;
mod admin;

impl<'a, 'r> FromRequest<'a, 'r> for &'a User {
  type Error = Error;

  fn from_request(request: &'a Request<'r>) -> request::Outcome<&'a User, Self::Error> {
    let user_result = request.local_cache(|| -> Option<User> {
      let db = request.guard::<Database>().succeeded()?;
      let session = UserSession::from_cookies(&db, &mut request.cookies()).ok()?;
      session.user(&db).ok()
    });
    match user_result {
      Some(user) => Outcome::Success(&user),
      None => Outcome::Failure((Status::Unauthorized, Self::Error::SessionNotFound))
    }
  }
}

pub fn routes() -> Vec<Route> {
  [
    routes![
      index::index,
      login::login,
      logout::logout,
      error::error,
    ],
    admin::routes(),
  ].concat()
}

pub fn catchers() -> Vec<Catcher> {
  catchers![]
}