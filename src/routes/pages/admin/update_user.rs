use rocket::request::{Form, FromForm, FormItems};
use rocket::response::Redirect;
use mongodb::{bson, doc};
use mongodb::oid::ObjectId;
use crate::util::database::Database;

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