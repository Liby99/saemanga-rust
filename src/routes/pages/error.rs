use rocket_contrib::templates::Template;
use rocket::request::Request;
use rocket::response::Redirect;
use rocket::{Route, Catcher};
use enum_primitive::FromPrimitive;

use crate::util::Error;

pub fn routes() -> Vec<Route> {
  routes![
    error
  ]
}

pub fn catchers() -> Vec<Catcher> {
  catchers![
    internal_error,
    not_found,
  ]
}

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

#[catch(500)]
pub fn internal_error() -> Template {
  Template::render("error", ErrorData {
    code: 500,
    msg: "Internal Server Error",
    back: "/"
  })
}

#[catch(404)]
pub fn not_found(req: &Request) -> Template {
  Template::render("error", ErrorData {
    code: 404,
    msg: format!("Cannot find {}", req.uri()).as_str(),
    back: "/"
  })
}