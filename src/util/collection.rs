use mongodb::coll::Collection as MongoCollection;

use super::Database;

pub trait Collection {
  fn coll(conn: &Database) -> MongoCollection;
}