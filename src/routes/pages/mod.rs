use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};
use rocket::{Route, Catcher};

use crate::util::Database;
use crate::util::Error;
use crate::app::user::User;
use crate::app::session::Session;

mod index;
mod error;
mod user;
mod admin;

impl<'a, 'r> FromRequest<'a, 'r> for &'a User {
  type Error = Error;

  fn from_request(request: &'a Request<'r>) -> request::Outcome<&'a User, Self::Error> {
    let user_result = request.local_cache(|| -> Option<User> {
      let db = request.guard::<Database>().succeeded()?;
      let session = Session::from_cookies_and_touch(&db, &mut request.cookies()).ok()?;
      session.get_user_and_touch(&db).ok()
    });
    match user_result {
      Some(user) => Outcome::Success(&user),
      None => Outcome::Forward(())
    }
  }
}

pub fn routes() -> Vec<Route> {
  [
    routes![
      index::index,
      error::error,
    ],
    user::routes(),
    admin::routes(),
  ].concat()
}

pub fn catchers() -> Vec<Catcher> {
  catchers![]
}