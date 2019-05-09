use rocket_contrib::templates::Template;
use mongodb::{Bson, doc};
use crate::utilities::database::Database;

#[derive(Serialize)]
struct RegisteredUser {
  id: String,
  username: String,
  register_date_time: String,
}

#[derive(Serialize)]
struct AdminData {
  users: Vec<RegisteredUser>
}

#[get("/admin")]
pub fn admin(conn: Database) -> Template {
  let user_collection = &conn.collection("user_info");
  let cursor = user_collection.find(None, None).ok().expect("Failed to find users");
  let users = cursor.map(|result| {
    let doc = result.expect("Received network error during cursor operations.");
    let username = if let Some(&Bson::String(ref value)) = doc.get("username") {
      value.clone()
    } else {
      panic!("Not possible");
    };
    let id = if let Some(&Bson::ObjectId(ref obj_id)) = doc.get("_id") {
      obj_id.to_hex()
    } else {
      panic!("Oh No!");
    };
    let register_date_time = if let Some(&Bson::UtcDatetime(ref dt)) = doc.get("register_date_time") {
      dt.to_rfc3339()
    } else {
      panic!("Noooooo")
    };

    RegisteredUser { id, username, register_date_time }
  }).collect::<Vec<_>>();
  Template::render("admin", &AdminData { users: users })
}