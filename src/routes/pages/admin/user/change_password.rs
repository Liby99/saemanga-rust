use rocket_contrib::templates::Template;
use rocket::request::Form;
use rocket::response::Redirect;

use crate::util::database::Database;
use crate::app::user::User;
use super::super::AdminUser;

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

#[get("/admin/user/change_password/<id>")]
pub fn change_password_page(_user: AdminUser, conn: Database, id: String) -> Result<Template, Redirect> {
  match User::get_by_id(&conn, &id) {
    Ok(user) => {
      let data = PageData::from(user);
      Ok(Template::render("admin/change_password", &data))
    },
    Err(err) => Err(Redirect::to(format!("/admin/error?code={}", err as u32)))
  }
}

#[get("/admin/user/change_password/<_id>", rank=2)]
pub fn change_password_page_fail(_id: String) -> Redirect {
  Redirect::to("/admin/login")
}

#[derive(FromForm)]
pub struct ChangePasswordForm {
  id: String,
  new_password: String,
}

#[post("/admin/user/change_password", data="<data>")]
pub fn change_password_submit(_user: AdminUser, conn: Database, data: Form<ChangePasswordForm>) -> Redirect {
  match User::change_password(&conn, &data.id, &data.new_password) {
    Ok(()) => Redirect::to("/admin"),
    Err(err) => Redirect::to(format!("/admin/error?code={}", err as u32))
  }
}