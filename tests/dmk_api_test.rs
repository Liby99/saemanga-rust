use saemanga::api::dmk_api::*;

#[test]
fn test_fetch_manga_5893() {
  let dmk_id = String::from("5893");
  match fetch_manga_data(&dmk_id) {
    Ok(()) => {},
    Err(_) => assert!(false),
  };
}

#[test]
fn test_fetch_manga_kaikan_shirei() {
  let dmk_id = String::from("1237");
  match fetch_manga_data(&dmk_id) {
    Ok(()) => {},
    Err(_) => assert!(false),
  };
}

#[test]
fn test_fetch_manga_shingeki_no_kyoujin() {
  let dmk_id = String::from("1221");
  match fetch_manga_data(&dmk_id) {
    Ok(()) => {},
    Err(_) => assert!(false),
  };
}

#[test]
fn test_fetch_manga_one_piece() {
  let dmk_id = String::from("1152");
  match fetch_manga_data(&dmk_id) {
    Ok(()) => {},
    Err(_) => assert!(false),
  };
}