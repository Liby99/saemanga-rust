use rocket::Route;

mod add;
mod upsert;
mod setup;
mod check;

pub fn routes() -> Vec<Route> {
  routes![
    add::add,
    upsert::upsert,
    setup::setup,
    check::check,
  ]
}