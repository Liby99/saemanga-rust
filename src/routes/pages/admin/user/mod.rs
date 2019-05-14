use rocket::Route;

mod create;
mod remove;
mod change_password;
mod test_password;
mod setup;

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
  ]
}