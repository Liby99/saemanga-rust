use rocket::Route;

mod front_end;
mod dmk;

pub fn routes() -> Vec<Route> {
  [
    front_end::routes(),
    dmk::routes(),
  ].concat()
}