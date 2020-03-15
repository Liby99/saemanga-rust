use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::response::Redirect;
use rocket::Outcome;
use rocket::Route;

use crate::app::user::User;
use crate::util::Error;

mod error;
mod follow;
mod index;
mod latest;
mod login;
mod manga;
mod user;

pub struct AdminUser<'a>(&'a User);

impl<'a> AdminUser<'a> {
  pub fn user(&self) -> &'a User {
    self.0
  }
}

impl<'a, 'r> FromRequest<'a, 'r> for AdminUser<'a> {
  type Error = Error;

  fn from_request(request: &'a Request<'r>) -> request::Outcome<AdminUser<'a>, Self::Error> {
    let user = request.guard::<&User>()?;
    if user.is_admin() {
      Outcome::Success(AdminUser(user))
    } else {
      Outcome::Failure((Status::Unauthorized, Self::Error::NotAuthenticated))
    }
  }
}

#[get("/admin")]
pub fn root() -> Redirect {
  Redirect::to("/admin/index")
}

pub fn routes() -> Vec<Route> {
  [
    routes![
      root,
      index::index,
      index::index_fail,
      login::login,
      error::error,
    ],
    user::routes(),
    manga::routes(),
    latest::routes(),
    follow::routes(),
  ]
  .concat()
}
