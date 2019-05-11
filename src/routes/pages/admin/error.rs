use rocket_contrib::templates::Template;
use enum_primitive::FromPrimitive;

use crate::app::user_error::UserError;

#[derive(Serialize)]
struct ErrorData<'a> {
  code: i32,
  msg: &'a str,
}

#[get("/admin/error?<code>")]
pub fn error(code: Option<i32>) -> Template {
  let data : ErrorData = match code {
    Some(c) => match UserError::from_i32(c) {
      Some(err) => match err {
        UserError::DatabaseError => ErrorData { code: c, msg: "Database Access Error" },
        UserError::UserIdError => ErrorData { code: c, msg: "User Id Parsing Error" },
        UserError::UserNotFoundError => ErrorData { code: c, msg: "User Not Found" },
        UserError::UserDataError => ErrorData { code: c, msg: "User Data Parsing Error" },
        UserError::UserExistedError => ErrorData { code: c, msg: "User already exists" },
        UserError::InvalidUsername => ErrorData { code: c, msg: "Invalid username" },
        UserError::InvalidPassword => ErrorData { code: c, msg: "Invalid Passowrd" },
      },
      None => ErrorData { code: 0, msg: "Unknown Error" }
    },
    None => ErrorData { code: 0, msg: "Unknown Error" }
  };
  Template::render("admin/error", &data)
}