use rocket::Route;

mod index;
mod manga;
mod setting;

pub fn routes() -> Vec<Route> {
  [
    index::routes(),
    manga::routes(),
    setting::routes(),
  ].concat()
}