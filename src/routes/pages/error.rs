use rocket_contrib::templates::Template;
use rocket::response::Redirect;
use enum_primitive::FromPrimitive;

use crate::util::Error;

impl Error {
  pub fn redirect<'a>(&self, redir: Option<&'a str>) -> Redirect {
    Redirect::to(format!("/error?code={}{}", self.code(), match redir {
      Some(s) => format!("&redir={}", s),
      None => format!("")
    }))
  }
}

#[derive(Serialize)]
struct ErrorData<'a> {
  code: i32,
  msg: &'a str,
  back: &'a str,
}

#[get("/error?<code>&<redir>")]
pub fn error(code: Option<i32>, redir: Option<String>) -> Result<Template, Redirect> {
  let back = match redir { Some(url) => url, None => String::from("/") };
  let data : ErrorData = match code {
    Some(c) => match Error::from_i32(c) {
      Some(e) => ErrorData { code: e.code(), msg: e.msg(), back: back.as_str() },
      None => ErrorData { code: -2, msg: "Unknown Error Code", back: back.as_str() },
    },
    None => ErrorData { code: -1, msg: "Error Not Specified", back: back.as_str() }
  };
  Ok(Template::render("error", &data))
}