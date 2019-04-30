#![feature(try_trait)]
#![feature(type_alias_enum_variants)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
// #[macro_use] extern crate serde_derive;

extern crate lazy_static;

use rocket_contrib::templates::{Template};

pub mod app;
pub mod api;
pub mod routes;

#[database("mongodb")]
struct Database(mongodb::db::Database);

pub fn launch() {
  rocket::ignite()
    .attach(Template::fairing())
    .attach(Database::fairing())
    .mount("/", routes::routes())
    .register(routes::catchers())
    .launch();
}