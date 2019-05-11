use rocket::Route;

mod add;

pub fn routes() -> Vec<Route> {
  routes![
    add::add,
  ]
}