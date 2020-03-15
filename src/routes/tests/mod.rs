use rocket::Route;

mod dmk;
mod front_end;

pub fn routes() -> Vec<Route> {
  [front_end::routes(), dmk::routes()].concat()
}
