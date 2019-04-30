use rocket::{Route, Catcher};

mod index;

pub fn routes() -> Vec<Route> {
  routes![
    index::index,
  ]
}

pub fn catchers() -> Vec<Catcher> {
  catchers![]
}