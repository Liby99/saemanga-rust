use rocket::Route;

mod fe;

pub fn routes() -> Vec<Route> {
  [fe::routes()].concat()
}