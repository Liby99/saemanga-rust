use rocket::Route;

mod add;
mod check;
mod setup;
mod upsert;

pub fn routes() -> Vec<Route> {
  routes![
    add::add,
    upsert::upsert,
    setup::setup,
    check::check,
    check::check_fail,
  ]
}
