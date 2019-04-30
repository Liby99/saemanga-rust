#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::response::Redirect;
use rocket::http::{Cookies, Cookie};
use rocket_contrib::templates::{Template};
use rocket_contrib::databases::mongodb;
use mongodb::{bson, doc};
use mongodb::{ThreadedClient};
use mongodb::db::ThreadedDatabase;

use saemanga::app;

#[database("mongodb")]
struct SaemangaDatabase(mongodb::db::Database);

#[get("/")]
fn root() -> Redirect {
  Redirect::to("/index")
}

#[derive(Serialize)]
struct IndexTemplateContext {
  name: &'static str,
}

#[get("/")]
fn index(cookies: Cookies) -> Template {
  println!("{:?}", app::user_setting::extract_user_setting(&cookies));
  Template::render("index", &IndexTemplateContext {
    name: "Liby"
  })
}

#[get("/")]
fn hello_world() -> &'static str {
  "Hello, world!"
}

#[get("/db-fetch")]
fn db_fetch_test(conn: SaemangaDatabase) -> &'static str {
  let client = &conn.client;
  let db = client.db("saemanga");
  let coll = db.collection("test");
  let mut cursor = coll.find(Some(doc!{}), None).ok().expect("Failed to execute find.");
  let item = cursor.next();
  match item {
    Some(Ok(doc)) => {
      println!("{:?}", doc);
      "Get!"
    },
    _ => "Uh..."
  }
}

#[get("/db-insert")]
fn db_insert_test(conn: SaemangaDatabase) -> String {
  let coll = &conn.client.db("saemanga").collection("test");
  coll.insert_one(doc! {
    "a": 1,
    "b": 3.0,
    "name": "Liby"
  }, None).unwrap();
  String::from("Inserted...?")
}

#[get("/set-cookie-zero")]
fn set_cookie_zero(mut cookies: Cookies) -> String {
  cookies.add(Cookie::build("counter", "0").path("/").finish());
  String::from("Setting counter to 0")
}

#[get("/get-counter")]
fn get_counter(cookies: Cookies) -> String {
  match cookies.get("counter") {
    Some(c) => format!("Current counter is {}", c.value()),
    None => String::from("Cookie counter not found")
  }
}

#[get("/cookie-add-one")]
fn cookie_add_one(mut cookies: Cookies) -> String {
  match cookies.get("counter") {
    Some(c) => {
      let v = c.value().to_string();
      match v.parse::<i32>() {
        Ok(old_int_value) => {
          let new_int_value: i32 = old_int_value + 1;
          cookies.add(Cookie::build("counter", new_int_value.to_string()).path("/").finish());
          format!("Incrementing counter {} to {}", old_int_value, new_int_value)
        },
        _ => {
          cookies.add(Cookie::new("counter", "1"));
          format!("Invalid counter {}. Setting it to 1", v)
        }
      }
    },
    _ => {
      cookies.add(Cookie::build("counter", "1").path("/").finish());
      format!("Counter not found. Setting it to 1")
    }
  }
}

#[test]
#[ignore]
fn rocket_test() {
  rocket::ignite()
    .attach(Template::fairing())
    .attach(SaemangaDatabase::fairing())
    .mount("/", routes![root])
    .mount("/index", routes![index])
    .mount("/test", routes![
      db_fetch_test,
      db_insert_test,
      get_counter,
      set_cookie_zero,
      cookie_add_one
    ])
    .mount("/hello-world", routes![hello_world])
    .launch();
}