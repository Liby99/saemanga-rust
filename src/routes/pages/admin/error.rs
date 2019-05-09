use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct ErrorData<'a> {
  code: u32,
  msg: &'a str,
}

#[get("/admin/error?<code>")]
pub fn error(code: Option<u32>) -> Template {
  let data : ErrorData = match code {
    Some(1000) => ErrorData { code: 1000, msg: "Database Access Error" },
    Some(1001) => ErrorData { code: 1001, msg: "Cannot create user because username already exists" },
    _ => ErrorData { code: 0, msg: "Unknown Error" }
  };
  Template::render("admin/error", &data)
}