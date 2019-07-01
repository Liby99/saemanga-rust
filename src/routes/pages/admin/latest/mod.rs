use rocket::Route;

mod fetch_overall;
mod fetch_genres;
mod fetch_ended;
mod fetch_oldest_updating;

pub fn routes() -> Vec<Route> {
  routes![
    fetch_overall::fetch_overall,
    fetch_genres::fetch_genres,
    fetch_ended::fetch_ended,
    fetch_oldest_updating::fetch_oldest_updating,
  ]
}