use rocket::Route;

mod add;
mod setup;

pub fn routes() -> Vec<Route> {
  routes![
    add::add,
    setup::setup,
  ]
}