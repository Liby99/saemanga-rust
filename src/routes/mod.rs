use rocket::{Route, Catcher};
use rocket::response::Redirect;

mod pages;
mod ajax;

#[cfg(debug_assertions)]
mod tests;

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

    #[cfg(debug_assertions)] { tests::routes() },
  ].concat()
}

pub fn catchers() -> Vec<Catcher> {
  pages::catchers()
}