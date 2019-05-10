use crate::app::user::User;
use crate::util::database::Database;
use rocket::request::Form;
use rocket::response::Redirect;

#[derive(FromForm)]
pub struct RemoveUserForm {
  id: String,
}

#[post("/admin/user/remove", data="<info>")]
pub fn remove(conn: Database, info: Form<RemoveUserForm>) -> Redirect {
  match User::remove(&conn, &info.id) {
    Ok(_) => Redirect::to("/admin"),
    Err(err) => Redirect::to(format!("/admin/error?code={}", err as u32))
  }
}