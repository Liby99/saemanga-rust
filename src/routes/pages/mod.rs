use rocket::{Route, Catcher};

#[get("/")]
fn hello_world() -> &'static str {
  "Hello, world!"
}

pub fn routes() -> Vec<Route> {
  routes![]
}

pub fn catchers() -> Vec<Catcher> {
  catchers![]
}