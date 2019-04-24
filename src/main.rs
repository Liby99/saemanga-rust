#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket::Request;
use rocket::response::Redirect;
use rocket_contrib::templates::{Template, handlebars};

#[derive(Serialize)]
struct IndexTemplateContext {
  name: &'static str,
}

#[get("/")]
fn root() -> Redirect {
  Redirect::to("/index")
}

#[get("/")]
fn index() -> Template {
  Template::render("index", &IndexTemplateContext {
    name: "Liby"
  })
}

#[get("/")]
fn hello_world() -> &'static str {
  "Hello, world!"
}

fn main() {
  rocket::ignite()
    .attach(Template::fairing())
    .mount("/", routes![root])
    .mount("/index", routes![index])
    .mount("/hello-world", routes![hello_world])
    .launch();
}