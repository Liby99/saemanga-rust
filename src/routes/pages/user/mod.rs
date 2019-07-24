use rocket::Route;

mod login;
mod register;
mod change_password;
mod logout;
mod migrate;

pub fn routes() -> Vec<Route> {
  routes![
    login::login,
    register::register,
    change_password::change_password,
    logout::logout,
    migrate::migrate,
  ]
}