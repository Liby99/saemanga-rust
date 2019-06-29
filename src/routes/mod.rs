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
  [routes![root], pages::routes(), ajax::routes(), tests::routes()].concat()
}

pub fn catchers() -> Vec<Catcher> {
  pages::catchers()
}