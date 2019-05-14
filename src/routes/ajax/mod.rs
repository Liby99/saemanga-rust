use rocket::Route;

mod setting;

pub fn routes() -> Vec<Route> {
  [
    setting::routes()
  ].concat()
}