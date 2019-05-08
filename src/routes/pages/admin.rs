use rocket::request::{Form, FromForm, FormItems};
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use mongodb::{Bson, bson, doc};
use mongodb::oid::ObjectId;
use crate::utilities::database::Database;

#[derive(Serialize)]
struct RegisteredUser {
  id: String,
  username: String,
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

    RegisteredUser { id: id, username: username }
  }).collect::<Vec<_>>();
  Template::render("admin", &AdminData { users: users })
}

#[derive(FromForm)]
pub struct User {
  username: String,
  password: String,
}

#[post("/admin/user/create", data="<info>")]
pub fn create_user(conn: Database, info: Form<User>) -> Redirect {
  println!("Testing, received username {}, password {}", info.username, info.password);
  let user_collection = &conn.collection("user_info");
  user_collection.insert_one(doc! {
    "username": info.username.as_str(),
    "password": info.password.as_str(),
  }, None).unwrap();
  Redirect::to("/admin")
}

#[derive(Debug)]
pub enum UpdateMethod {
  ChangePassword,
  Remove,
}

pub struct UpdateUser {
  method: UpdateMethod,
  id: String,
}

impl<'f> FromForm<'f> for UpdateUser {
  // In practice, we'd use a more descriptive error type.
  type Error = &'f str;

  fn from_form(items: &mut FormItems<'f>, _: bool) -> Result<UpdateUser, &'f str> {
    let mut method = None;
    let mut id = None;

    for item in items {
      match item.key.as_str() {
        "method" => {
          println!("Method Get!");
          method = match item.value.url_decode().map_err(|_| "Cannot decode method")?.as_str() {
            "remove" => Some(UpdateMethod::Remove),
            "change_password" => Some(UpdateMethod::ChangePassword),
            _ => None
          };
        },
        "user_id" => {
          println!("User Id Get!");
          id = Some(String::from(item.value.url_decode().map_err(|_| "Cannot decode id")?));
        },
        _ => ()
      }
    }

    match method {
      Some(m) => match id {
        Some(i) => Ok(UpdateUser { method: m, id: i }),
        None => Err("No id exists")
      },
      _ => Err("No Method")
    }
  }
}

#[post("/admin/user/update", data="<data>")]
pub fn update_user(conn: Database, data: Form<UpdateUser>) -> Redirect {
  println!("Updating user (id: {}) with method {:?}", data.id, data.method);
  match data.method {
    UpdateMethod::Remove => {
      let user_collection = &conn.collection("user_info");
      user_collection.delete_one(doc! {
        "_id": ObjectId::with_string(data.id.as_str()).unwrap(),
      }, None).unwrap();
      Redirect::to("/admin")
    },
    _ => Redirect::to("/admin")
  }
}