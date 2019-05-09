use bson::Bson;

use super::session_model::SessionModel;
use super::BsonConversionError;

#[derive(Serialize, Deserialize)]
pub struct UserModel {
  #[serde(rename="_id")] id: mongodb::oid::ObjectId,
  username: String,
  password: String,
  is_admin: bool,
  visit_count: u32,
  register_date_time: bson::UtcDateTime,
  last_visit_date_time: bson::UtcDateTime,
  sessions: Vec<SessionModel>,
}

impl UserModel {
  pub fn from(bson: Bson) -> Result<Self, BsonConversionError> {
    match bson::from_bson::<UserModel>(bson) {
      Ok(model) => Ok(model),
      Err(_) => Err(BsonConversionError)
    }
  }
}