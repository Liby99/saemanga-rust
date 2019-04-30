#![feature(try_trait)]
#![feature(type_alias_enum_variants)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

extern crate lazy_static;

use rocket::fairing::Fairing;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::{Template, handlebars};
use handlebars::{Helper, Handlebars, Context, RenderContext, Output, HelperResult, JsonRender};

pub mod app;
pub mod api;
pub mod routes;

#[database("mongodb")]
struct Database(mongodb::db::Database);

fn concatenate_helper(
  h: &Helper,
  _: &Handlebars,
  _: &Context,
  _: &mut RenderContext,
  out: &mut Output
) -> HelperResult {
  for param in h.params() {
    out.write(&param.value().render())?;
  }
  Ok(())
}

fn template() -> impl Fairing {
  Template::custom(|engines| {
    engines.handlebars.register_helper("concat", Box::new(concatenate_helper));
  })
}

fn database() -> impl Fairing {
  Database::fairing()
}

pub fn launch() {

  // Variables
  let public_path = concat!(env!("CARGO_MANIFEST_DIR"), "/public");

  // Start the server
  rocket::ignite()
    .attach(template())
    .attach(database())
    .mount("/", routes::routes())
    .mount("/", StaticFiles::from(public_path))
    .register(routes::catchers())
    .launch();
}