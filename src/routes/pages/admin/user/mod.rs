use rocket::Route;

mod create;
mod remove;
mod change_password;

pub fn routes() -> Vec<Route> {
  routes![
    create::create,
    remove::remove,
    change_password::change_password_page,
    change_password::change_password_submit,
  ]
}