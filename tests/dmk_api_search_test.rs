use saemanga::app::dmk::*;

#[test]
fn search_test() {
  match search(&String::from("我們")) {
    Ok(ids) => println!("{:?}", ids),
    Err(_) => assert!(false),
  }
}
