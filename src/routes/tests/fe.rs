use rocket::Route;
use rocket_contrib::templates::Template;

#[get("/tests/fe/ctrl_rm")]
fn ctrl_rm() -> Template {
  Template::render("tests/fe/ctrl_rm", ())
}

pub fn routes() -> Vec<Route> {
  routes![
    ctrl_rm,
  ]
}