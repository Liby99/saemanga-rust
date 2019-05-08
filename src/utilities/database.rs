use rocket::fairing::Fairing;

#[database("mongodb")]
struct Database(mongodb::db::Database);

pub fn database() -> impl Fairing {
  Database::fairing()
}