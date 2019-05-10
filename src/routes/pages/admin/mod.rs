use rocket::Route;
use rocket::response::Redirect;

mod index;
mod error;
mod user;

#[get("/admin")]
pub fn root() -> Redirect {
  Redirect::to("/admin/index")
}

pub fn routes() -> Vec<Route> {
  [
    routes![
      root,
      index::index,
      error::error
    ],
    user::routes()
  ].concat()
}