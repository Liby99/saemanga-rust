#![feature(try_trait)]
#![feature(type_alias_enum_variants)]
#![feature(result_map_or_else)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate enum_primitive;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

extern crate bson;
extern crate crypto;
extern crate lazy_static;
extern crate time;

#[macro_use] extern crate collection;
#[macro_use] extern crate cookie_value;

pub mod app;
pub mod routes;
pub mod util;

pub fn launch() {
  rocket::ignite()
    .attach(util::template())
    .attach(util::database())
    .mount("/", routes::routes())
    .mount("/", util::static_files())
    .register(routes::catchers())
    .launch();
}