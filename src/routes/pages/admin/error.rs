use rocket_contrib::templates::Template;
use rocket::response::Redirect;
use enum_primitive::FromPrimitive;

use crate::util::Error;

impl Error {
  pub fn redirect_to_admin(&self) -> Redirect {
    Redirect::to(format!("/admin/error?code={}", self.code()))
  }
}

#[derive(Serialize)]
struct ErrorData<'a> {
  code: i32,
  msg: &'a str,
}

#[get("/admin/error?<code>")]
pub fn error(code: Option<i32>) -> Template {
  let data : ErrorData = match code {
    Some(c) => match Error::from_i32(c) {
      Some(e) => ErrorData { code: e.code(), msg: e.msg() },
      None => ErrorData { code: -2, msg: "Unknown Error Code" },
    },
    None => ErrorData { code: -1, msg: "Error Not Specified" }
  };
  Template::render("admin/error", &data)
}