use saemanga::api::dmk_api::*;

#[test]
fn test_fetch_manga_5893() {
  let dmk_id = String::from("5893");
  match fetch_manga_data(&dmk_id) {
    Ok(()) => {},
    Err(_) => assert!(false),
  };
}