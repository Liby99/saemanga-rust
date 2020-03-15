use rocket::Route;

mod index;
mod manga;
mod setting;
mod user;

pub fn routes() -> Vec<Route> {
  [
    index::routes(),
    manga::routes(),
    user::routes(),
    setting::routes(),
  ]
  .concat()
}
