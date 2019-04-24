#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::Request;
use rocket::response::Redirect;
use rocket_contrib::templates::{Template, handlebars};
use rocket_contrib::databases::mongodb;

#[database("mongodb")]
struct SaemangaDatabase(mongodb::db::Database);

#[get("/")]
fn root() -> Redirect {
  Redirect::to("/index")
}

#[derive(Serialize)]
struct IndexTemplateContext {
  name: &'static str,
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
    .attach(SaemangaDatabase::fairing())
    .mount("/", routes![root])
    .mount("/index", routes![index])
    .mount("/hello-world", routes![hello_world])
    .launch();
}