use rocket::Route;

mod add;
mod setup;
mod check;

pub fn routes() -> Vec<Route> {
  routes![
    add::add,
    setup::setup,
    check::check,
  ]
}