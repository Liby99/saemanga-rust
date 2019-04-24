#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::Request;
use rocket::response::Redirect;
use rocket_contrib::templates::{Template, handlebars};
use rocket_contrib::databases::mongodb;
use mongodb::{Bson, bson, doc};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

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

#[get("/db-fetch")]
fn db_fetch_test(conn: SaemangaDatabase) -> &'static str {
  let client = &conn.client;
  let db = client.db("saemanga");
  let coll = db.collection("test");
  let mut cursor = coll.find(Some(doc!{}), None).ok().expect("Failed to execute find.");
  let item = cursor.next();
  match item {
    Some(Ok(doc)) => {
      println!("{:?}", doc);
      "Get!"
    },
    _ => "Uh..."
  }
}

fn main() {
  rocket::ignite()
    .attach(Template::fairing())
    .attach(SaemangaDatabase::fairing())
    .mount("/", routes![root])
    .mount("/index", routes![index])
    .mount("/test", routes![db_fetch_test])
    .mount("/hello-world", routes![hello_world])
    .launch();
}