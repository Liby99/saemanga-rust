use rocket::Route;
use rocket::response::Redirect;

mod index;
mod create_user;
mod update_user;
mod change_password;
mod error;

#[get("/admin")]
pub fn root() -> Redirect {
  Redirect::to("/admin/index")
}

pub fn routes() -> Vec<Route> {
  routes![
    root,
    index::index,
    create_user::create_user,
    update_user::update_user,
    change_password::change_password,
    error::error,
  ]
}