use rocket_contrib::templates::Template;
use rocket::response::Redirect;

use crate::util::database::Database;
use crate::app::user::User;

#[derive(Serialize, Deserialize)]
struct RegisteredUser {
  id: String,
  username: String,
  register_date_time: String,
  visit_count: i32,
}

impl From<&User> for RegisteredUser {
  fn from(user: &User) -> Self {
    RegisteredUser {
      id: user.id().to_hex(),
      username: user.username().clone(),
      register_date_time: user.register_date_time().to_rfc3339(),
      visit_count: user.visit_count()
    }
  }
}

#[derive(Serialize)]
struct AdminData {
  users: Vec<RegisteredUser>
}

#[get("/admin/index")]
pub fn index(conn: Database, user: &User) -> Result<Template, Redirect> {
  let users_res = User::get_all(&conn).map(|users: Vec<User>| {
    users.iter().map(|u| RegisteredUser::from(u)).collect()
  });
  match users_res {
    Ok(users) => Ok(Template::render("admin/index", &AdminData { users: users })),
    Err(err) => Err(Redirect::to(format!("/admin/error?code={}", err as u32)))
  }
}