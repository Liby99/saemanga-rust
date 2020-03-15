use rocket::Route;

mod setup;

pub fn routes() -> Vec<Route> {
  routes![setup::setup,]
}
