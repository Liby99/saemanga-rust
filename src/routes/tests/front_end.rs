use rocket::Route;
use rocket_contrib::templates::Template;

#[get("/tests/front_end/ctrl_rm")]
fn ctrl_rm() -> Template {
  Template::render("tests/front_end/ctrl_rm", ())
}

#[get("/tests/front_end/ctrl_add")]
fn ctrl_add() -> Template {
  Template::render("tests/front_end/ctrl_add", ())
}

pub fn routes() -> Vec<Route> {
  routes![ctrl_rm, ctrl_add,]
}
