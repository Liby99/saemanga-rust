use rocket::Route;

mod fetch_overall;
mod fetch_genres;

pub fn routes() -> Vec<Route> {
  routes![
    fetch_overall::fetch_overall,
    fetch_genres::fetch_genres
  ]
}