use rocket_contrib::templates::Template;
use rocket::request::Form;
use rocket::response::Redirect;

use crate::app::user::User;
use crate::util::database::Database;

#[derive(Serialize)]
struct PageData {
  id: String,
  username: String,
}

impl From<User> for PageData {
  fn from(user: User) -> Self {
    PageData {
      id: user.id().to_hex(),
      username: user.username().clone(),
    }
  }
}

#[get("/admin/user/test_password/<id>")]
pub fn test_password_page(conn: Database, id: String) -> Result<Template, Redirect> {
  match User::get_by_id(&conn, &id) {
    Ok(user) => {
      let data = PageData::from(user);
      Ok(Template::render("admin/test_password", &data))
    },
    Err(err) => Err(Redirect::to(format!("/admin/error?code={}", err as u32)))
  }
}

#[derive(FromForm)]
pub struct TestPasswordForm {
  id: String,
  password: String,
}

#[derive(Serialize)]
struct TestPasswordResult {
  id: String,
  passed: bool,
}

#[post("/admin/user/test_password", data="<data>")]
pub fn test_password_submit(conn: Database, data: Form<TestPasswordForm>) -> Result<Template, Redirect> {
  match User::is_password_match(&conn, &data.id, &data.password) {
    Ok(passed) => Ok(Template::render("admin/test_password_result", TestPasswordResult { id: data.id.clone(), passed })),
    Err(err) => Err(Redirect::to(format!("/admin/error?code={}", err as u32)))
  }
}