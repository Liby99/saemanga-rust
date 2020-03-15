use rocket::Route;

mod fetch_ended;
mod fetch_genres;
mod fetch_loved;
mod fetch_oldest_updating;
mod fetch_overall;

pub fn routes() -> Vec<Route> {
  routes![
    fetch_overall::fetch_overall,
    fetch_genres::fetch_genres,
    fetch_ended::fetch_ended,
    fetch_oldest_updating::fetch_oldest_updating,
    fetch_loved::fetch_loved,
  ]
}
