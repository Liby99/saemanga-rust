use rocket::{Route, Catcher};

mod index;
mod admin;

pub fn routes() -> Vec<Route> {
  routes![
    index::index,
    admin::admin,
    admin::create_user,
    admin::update_user,
  ]
}

pub fn catchers() -> Vec<Catcher> {
  catchers![]
}