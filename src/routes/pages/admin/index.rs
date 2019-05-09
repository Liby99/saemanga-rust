use rocket_contrib::templates::Template;
use mongodb::doc;
use crate::util::database::Database;
use mongodb::ordered::OrderedDocument;

#[derive(Serialize, Deserialize)]
struct RegisteredUser {
  id: String,
  username: String,
  register_date_time: String,
}

impl From<User> for RegisteredUser {
  fn from(user: User) -> Self {
    RegisteredUser {
      id: user.id.to_hex(),
      username: user.username,
      register_date_time: user.register_date_time.to_rfc3339(),
    }
  }
}

#[derive(Serialize, Deserialize)]
struct User {
  #[serde(rename = "_id")]
  id: bson::oid::ObjectId,
  username: String,
  register_date_time: bson::UtcDateTime,
}

impl From<OrderedDocument> for User {
  fn from(doc: OrderedDocument) -> Self {
    match bson::from_bson::<User>(bson::Bson::Document(doc)) {
      Ok(user) => user,
      Err(_) => panic!("Not able to deserialize object to registered user")
    }
  }
}

#[derive(Serialize)]
struct AdminData {
  users: Vec<RegisteredUser>
}

#[get("/admin/index")]
pub fn index(conn: Database) -> Template {
  let user_collection = &conn.collection("user_info");
  let cursor = user_collection.find(None, None).ok().expect("Failed to find users");
  let users = cursor.map(|result| {
    let doc : OrderedDocument = result.expect("Received network error during cursor operations.");
    RegisteredUser::from(User::from(doc))
  }).collect::<Vec<_>>();
  Template::render("admin/index", &AdminData { users: users })
}