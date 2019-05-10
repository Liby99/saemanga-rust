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
    Some(1001) => ErrorData { code: 1001, msg: "User Data Parsing Error" },
    Some(1002) => ErrorData { code: 1002, msg: "User Id Parsing Error" },
    Some(1003) => ErrorData { code: 1003, msg: "User Not Found Error" },
    Some(1004) => ErrorData { code: 1004, msg: "User Already Existed" },
    Some(1005) => ErrorData { code: 1005, msg: "Invalid Password" },
    Some(1006) => ErrorData { code: 1006, msg: "Invalid Username" },
    _ => ErrorData { code: 0, msg: "Unknown Error" }
  };
  Template::render("admin/error", &data)
}