use rocket::{Route, Catcher};

pub mod pages;
pub mod ajax;

pub fn routes() -> Vec<Route> {
  [pages::routes(), ajax::routes()].concat()
}

pub fn catchers() -> Vec<Catcher> {
  pages::catchers()
}