use saemanga::app::dmk::*;
use saemanga::app::genre::*;

#[test]
fn test_fetch_latest() {
  match fetch_latest_manga() {
    Ok(data) => println!("{:?}", data),
    Err(_) => assert!(false),
  }
}

#[test]
fn test_fetch_latest_of_genre() {
  match Genre::for_id("gakuen") {
    Some(genre) => match fetch_latest_manga_with_genre(genre) {
      Ok(data) => println!("{:?}", data),
      Err(_) => assert!(false),
    },
    None => println!("Genre gakuen not found"),
  }
}
