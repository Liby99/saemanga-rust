use rocket::Route;

mod change_password;
mod login;
mod logout;
mod migrate;
mod register;

pub fn routes() -> Vec<Route> {
  routes![
    login::login,
    register::register,
    change_password::change_password,
    logout::logout,
    migrate::migrate,
  ]
}
