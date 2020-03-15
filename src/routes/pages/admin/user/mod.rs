use rocket::Route;

mod change_password;
mod create;
mod remove;
mod session;
mod setup;
mod test_password;

pub fn routes() -> Vec<Route> {
  routes![
    setup::setup,
    create::create,
    remove::remove,
    change_password::change_password_page,
    change_password::change_password_page_fail,
    change_password::change_password_submit,
    test_password::test_password_page,
    test_password::test_password_page_fail,
    test_password::test_password_submit,
    session::purge,
  ]
}
