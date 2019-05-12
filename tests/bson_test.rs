use saemanga;
use saemanga::app::dmk;
use saemanga::app::dmk_id_base::*;
use saemanga::app::manga_status::*;
use saemanga::app::manga_data::*;
use saemanga::app::user::*;
use saemanga::app::manga::*;
use mongodb::{Bson, bson, doc};

#[test]
fn dmk_id_base_deser_test() {
  let dib = bson::from_bson::<DmkIdBase>(bson::Bson::Document(doc! {
    "version": "V10",
    "dmk_id_web": "web3",
    "dmk_id_home": "home19534",
  }));
  println!("{:?}", dib);
}

#[test]
fn dmk_id_base_ser_test() {
  let dib : DmkIdBase = DmkIdBase::V10 {
    dmk_id_web: String::from("web3"),
    dmk_id_home: String::from("home19534")
  };
  let bs = bson::to_bson(&dib);
  println!("{:?}", bs);
}

#[test]
fn status_ser_test() {
  let s = MangaStatus::Ended;
  let bs = bson::to_bson(&s);
  println!("{:?}", bs);
}

#[test]
fn status_deser_test() {
  let dib = bson::from_bson::<MangaStatus>(Bson::String(String::from("ended")));
  println!("{:?}", dib);
}

#[test]
fn manga_deser_test() {
  let dib = bson::from_bson::<MangaData>(Bson::Document(doc! {
    "dmk_id": "1356",
    "dmk_id_base": {
      "version": "V10",
      "dmk_id_web": "web3",
      "dmk_id_home": "home19535",
    },
    "title": "To Love Lu",
    "description": "Baka Yuki Rito!",
    "author": "Someone",
    "tags": [
      "love",
      "sexy",
    ],
    "genre": "magic",
    "status": "ended",
    "episodes": [{
      "index": 0,
      "is_book": true,
      "episode": 1,
      "num_pages": 124,
      "dmk_directory": "/comic/190268205963.html"
    }, {
      "index": 1,
      "is_book": false,
      "episode": 10,
      "num_pages": 19,
      "dmk_directory": "/comic/190268205966.html"
    }]
  }));
  println!("{:?}", dib);
}

#[test]
fn user_ser_test() {
  let username = String::from("test_user");
  let password = String::from("12345678");
  let user_res = User::new(&username, &password);
  match user_res {
    Ok(user) => println!("{:?}", bson::to_bson(&user)),
    Err(err) => println!("User Construction Error: {:?}", err)
  }
}

#[test]
fn manga_wrapper_ser_test() {
  match dmk::fetch_manga_data(&String::from("1234")) {
    Ok(manga) => match Manga::new(&manga) {
      Ok(wrapped) => match bson::to_bson(&wrapped) {
        Ok(doc) => println!("Serialized: {:?}", doc),
        Err(err) => println!("Unable to serialize: {:?}", err)
      },
      Err(err) => println!("Manga wrapping error: {:?}", err)
    },
    Err(err) => println!("Fetch manga 1234 failed: {:?}", err)
  }
}