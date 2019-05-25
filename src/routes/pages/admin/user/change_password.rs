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

#[get("/admin/user/change_password/<id>")]
pub fn change_password_page(admin: AdminUser, conn: Database, id: String) -> Result<Template, Redirect> {
  match User::get_by_id(&conn, &id) {
    Ok(user) => {
      let data = PageData {
        admin_username: admin.user().username(),
        id: &id,
        username: user.username(),
      };
      Ok(Template::render("admin/user/change_password", &data))
    },
    Err(err) => Err(err.redirect_to_admin())
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
  match User::change_password_by_id(&conn, &data.id, &data.new_password) {
    Ok(()) => Redirect::to("/admin"),
    Err(err) => err.redirect_to_admin()
  }
}