use rocket_contrib::templates::Template;
use rocket::request::Form;
use rocket::response::Redirect;

use crate::util::Database;
use crate::app::user::User;
use super::super::AdminUser;

#[derive(Serialize)]
struct PageData<'a> {
  admin_username: &'a String,
  id: &'a String,
  username: &'a String,
}

#[get("/admin/user/test_password/<id>")]
pub fn test_password_page(admin: AdminUser, conn: Database, id: String) -> Result<Template, Redirect> {
  match User::get_by_id(&conn, &id) {
    Ok(user) => {
      let data = PageData {
        admin_username: admin.user().username(),
        id: &id,
        username: user.username(),
      };
      Ok(Template::render("admin/user/test_password", &data))
    },
    Err(err) => Err(Redirect::to(format!("/admin/error?code={}", err as u32)))
  }
}

#[get("/admin/user/test_password/<_id>", rank=2)]
pub fn test_password_page_fail(_id: String) -> Redirect {
  Redirect::to("/admin/login")
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
pub fn test_password_submit(_user: AdminUser, conn: Database, data: Form<TestPasswordForm>) -> Result<Template, Redirect> {
  match User::get_by_id(&conn, &data.id) {
    Ok(user) => Ok(Template::render("admin/user/test_password_result", TestPasswordResult {
      id: data.id.clone(),
      passed: User::is_password_match(&user, &data.password)
    })),
    Err(err) => Err(Redirect::to(format!("/admin/error?code={}", err as u32)))
  }
}