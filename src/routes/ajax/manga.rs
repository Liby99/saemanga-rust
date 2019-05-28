use rocket::Route;
use rocket_contrib::json::Json;

use crate::util::Database;
use crate::app::user::User;
use crate::app::follow::Follow;

pub fn routes() -> Vec<Route> {
  routes![
    like,
    unlike,
    unfollow,
  ]
}

#[derive(Serialize)]
struct Response<'a> {
  success: bool,
  message: &'a str
}

#[get("/ajax/manga/like?<dmk_id>")]
fn like<'a>(conn: Database, user: &User, dmk_id: String) -> Json<Response<'a>> {
  match Follow::like(&conn, user, &dmk_id) {
    Ok(_) => Json(Response { success: true, message: "" }),
    Err(err) => Json(Response { success: false, message: err.msg() })
  }
}

#[get("/ajax/manga/unlike?<dmk_id>")]
fn unlike<'a>(conn: Database, user: &User, dmk_id: String) -> Json<Response<'a>> {
  match Follow::unlike(&conn, user, &dmk_id) {
    Ok(_) => Json(Response { success: true, message: "" }),
    Err(err) => Json(Response { success: false, message: err.msg() })
  }
}

#[get("/ajax/manga/unfollow?<dmk_id>")]
fn unfollow<'a>(conn: Database, user: &User, dmk_id: String) -> Json<Response<'a>> {
  match Follow::unfollow(&conn, user, &dmk_id) {
    Ok(_) => Json(Response { success: true, message: "" }),
    Err(err) => Json(Response { success: false, message: err.msg() })
  }
}