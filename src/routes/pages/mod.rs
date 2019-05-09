use rocket::{Route, Catcher};

mod index;
mod admin;

pub fn routes() -> Vec<Route> {
  [routes![
    index::index,
  ],
  admin::routes()].concat()
}

pub fn catchers() -> Vec<Catcher> {
  catchers![]
}