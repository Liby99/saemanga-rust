#![feature(try_trait)]
#![feature(type_alias_enum_variants)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

extern crate crypto;
extern crate lazy_static;

pub mod app;
pub mod api;
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