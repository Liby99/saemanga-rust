use rocket::Route;

mod index;
mod manga;
mod user;
mod setting;

pub fn routes() -> Vec<Route> {
  [
    index::routes(),
    manga::routes(),
    user::routes(),
    setting::routes(),
  ].concat()
}