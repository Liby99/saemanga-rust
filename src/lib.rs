#![feature(try_trait)]
#![feature(type_alias_enum_variants)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

extern crate lazy_static;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::{Template};

pub mod app;
pub mod api;
pub mod routes;

#[database("mongodb")]
struct Database(mongodb::db::Database);

pub fn launch() {

  // Variables
  let public_path = concat!(env!("CARGO_MANIFEST_DIR"), "/public");

  // Start the server
  rocket::ignite()
    .attach(Template::fairing())
    .attach(Database::fairing())
    .mount("/", routes::routes())
    .mount("/", StaticFiles::from(public_path))
    .register(routes::catchers())
    .launch();
}