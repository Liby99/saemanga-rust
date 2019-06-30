use rocket::Route;
use rocket_contrib::templates::Template;

#[get("/tests/fe/ctrl_rm")]
fn ctrl_rm() -> Template {
  Template::render("tests/fe/ctrl_rm", ())
}

#[get("/tests/fe/ctrl_add")]
fn ctrl_add() -> Template {
  Template::render("tests/fe/ctrl_add", ())
}

pub fn routes() -> Vec<Route> {
  routes![
    ctrl_rm,
    ctrl_add,
  ]
}