use rocket::{Route, Catcher};
use rocket::response::Redirect;

pub mod pages;
pub mod ajax;
pub mod tests;

#[get("/")]
fn root() -> Redirect {
  Redirect::to("/index")
}

pub fn routes() -> Vec<Route> {
  [
    // Root route
    routes![root],

    // Basic routes
    pages::routes(),
    ajax::routes(),

    // Testing routes
    if cfg!(debug_assertions) {
      [tests::routes()].concat()
    } else {
      // Prod routes
      routes![]
    },
  ].concat()
}

pub fn catchers() -> Vec<Catcher> {
  pages::catchers()
}