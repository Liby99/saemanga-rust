use crate::util::database::Database;
use rocket::request::Form;
use rocket::response::Redirect;
use mongodb::{bson, doc};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use chrono::Utc;

#[derive(FromForm)]
pub struct User {
  username: String,
  password: String,
}

#[post("/admin/user/create", data="<info>")]
pub fn create_user(conn: Database, info: Form<User>) -> Redirect {
  let mut hasher = Sha256::new();
  hasher.input_str(&info.password.as_str());
  println!("Testing, received username {}, password {}", info.username, info.password);
  let user_collection = &conn.collection("user_info");
  match user_collection.insert_one(doc! {
    "username": info.username.as_str(),
    "password": hasher.result_str(),
    "register_date_time": Utc::now(),
    "login_count": 0,
  }, None) {
    Ok(result) => {
      match result.inserted_id {
        Some(_) => Redirect::to("/admin"),
        None => Redirect::to("/admin/error?code=1001")
      }
    },
    Err(_) => Redirect::to("/admin/error?code=1000")
  }
}