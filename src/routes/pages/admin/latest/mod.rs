use rocket::Route;

mod fetch_overall;

pub fn routes() -> Vec<Route> {
  routes![
    fetch_overall::fetch_overall,
  ]
}