use scraper::Html;

#[test]
fn test_cartoonmad_homepage() {
  let res = reqwest::get("https://cartoonmad.com");
  match res {
    Ok(mut sth) => {
      match sth.text() {
        Ok(html_txt) => println!("{:?}", html_txt),
        Err(_) => println!("Parse text error")
      }
    },
    Err(_) => {
      println!("Get error")
    }
  }
}