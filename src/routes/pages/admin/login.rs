use rocket_contrib::templates::Template;

#[get("/admin/login")]
pub fn login() -> Template {
  Template::render("admin/login", ())
}
