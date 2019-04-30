use rocket_contrib::templates::{Template};

#[derive(Serialize)]
struct TemplateData {
  title: String,
  name: String,
}

#[get("/index")]
pub fn index() -> Template {
  Template::render("index", &TemplateData {
    title: String::from("saemanga"),
    name: String::from("Liby"),
  })
}