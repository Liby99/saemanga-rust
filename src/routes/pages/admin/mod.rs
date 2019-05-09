use rocket::Route;

mod admin;
mod create_user;
mod update_user;
mod error;

pub fn routes() -> Vec<Route> {
  routes![
    admin::admin,
    create_user::create_user,
    update_user::update_user,
    error::error,
  ]
}