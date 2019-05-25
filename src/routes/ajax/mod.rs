use rocket::Route;

mod index;
mod setting;

pub fn routes() -> Vec<Route> {
  [
    index::routes(),
    setting::routes(),
  ].concat()
}