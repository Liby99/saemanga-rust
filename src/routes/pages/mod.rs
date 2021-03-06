use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use rocket::{Catcher, Route};

use crate::app::session::Session;
use crate::app::user::User;
use crate::app::user_setting::UserSetting;
use crate::util::Database;
use crate::util::Error;

mod admin;
mod error;
mod index;
mod manga;
mod user;

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
      None => Outcome::Forward(()),
    }
  }
}

impl<'a, 'r> FromRequest<'a, 'r> for UserSetting {
  type Error = Error;

  fn from_request(request: &'a Request<'r>) -> request::Outcome<UserSetting, Self::Error> {
    Outcome::Success(UserSetting::from_cookies(&request.cookies()))
  }
}

pub fn routes() -> Vec<Route> {
  [
    index::routes(),
    manga::routes(),
    error::routes(),
    user::routes(),
    admin::routes(),
  ]
  .concat()
}

pub fn catchers() -> Vec<Catcher> {
  error::catchers()
}
