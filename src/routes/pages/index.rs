use rocket_contrib::templates::{Template};

use crate::app::genre::*;

#[derive(Serialize)]
struct TemplateData {
  title: String,
  username: String,
  logged_in: bool,
  genres: &'static [&'static Genre; 14],
  parent: &'static str,
}

#[get("/index")]
pub fn index() -> Template {
  Template::render("index", &TemplateData {
    title: String::from("saemanga"),
    username: String::from("Liby99"),
    logged_in: true,
    genres: &ALL_GENRES,
    parent: "layout/layout",
  })
}