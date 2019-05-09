#[derive(Serialize, Deserialize)]
pub struct SessionModel {
  session_id: mongodb::oid::ObjectId,
  start_date_time: bson::UtcDateTime,
  expire_date_time: bson::UtcDateTime,
}