use mongodb::db::ThreadedDatabase;
use mongodb::ThreadedClient;
use rocket::fairing::Fairing;

#[database("mongodb")]
pub struct Database(mongodb::db::Database);

impl Database {
  pub fn db(&self) -> mongodb::db::Database {
    self.client.db("saemanga")
  }

  pub fn collection(&self, name: &str) -> mongodb::coll::Collection {
    self.db().collection(&name)
  }
}

pub fn database() -> impl Fairing {
  Database::fairing()
}
