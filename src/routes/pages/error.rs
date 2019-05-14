use rocket_contrib::templates::Template;
use rocket::response::Redirect;
use enum_primitive::FromPrimitive;
use rocket::http::uri::Uri;

use crate::util::Error;

#[derive(Serialize)]
struct ErrorData<'a> {
  code: i32,
  msg: &'a str,
  back: &'a str,
}

#[get("/error?<code>&<redir>")]
pub fn error(code: Option<i32>, redir: Option<String>) -> Result<Template, Redirect> {
  let back = match redir {
    Some(url) => match Uri::percent_decode(url.as_bytes()) {
      Ok(decoded) => String::from(decoded),
      Err(_) => {
        return Err(Redirect::to(format!("/error?code={}", Error::DecodeUrlError.code())))
      }
    },
    None => String::from("/")
  };
  let data : ErrorData = match code {
    Some(c) => match Error::from_i32(c) {
      Some(e) => ErrorData { code: e.code(), msg: e.msg(), back: back.as_str() },
      None => ErrorData { code: -2, msg: "Unknown Error Code", back: back.as_str() },
    },
    None => ErrorData { code: -1, msg: "Error Not Specified", back: back.as_str() }
  };
  Ok(Template::render("error", &data))
}