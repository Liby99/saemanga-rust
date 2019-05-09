use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct Data {
  id: String,
  username: String,
}

#[get("/admin/user/change_password/<id>")]
pub fn change_password(id: String) -> Template {
  let data = Data { id, username: String::from("liby99") };
  Template::render("admin/change_password", &data)
}