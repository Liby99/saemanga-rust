use scraper::{Selector, Html};

#[test]
fn test_cartoonmad_homepage() {
  let res = reqwest::get("https://cartoonmad.com");
  match res {
    Ok(mut sth) => {
      match sth.text_with_charset("big5") {
        Ok(html_txt) => {
          let document = Html::parse_document(&html_txt);
          let selector = Selector::parse("body > table > tbody > tr:first-child > td:nth-child(2) > table > tbody > tr:nth-child(4) > td > table > tbody > tr:nth-child(2) > td:nth-child(2) > table > tbody").unwrap();
          let tbody_rows = document.select(&selector).next().unwrap();
          let snd_row_selector = Selector::parse("tr:nth-child(3) > td > a").unwrap();
          let snd_row = tbody_rows.select(&snd_row_selector);
          for element in snd_row {
            println!("{:?}", element.value().attr("href"));
          }
          let forth_row_selector = Selector::parse("tr:nth-child(5) > td > a").unwrap();
          let forth_row = tbody_rows.select(&forth_row_selector);
          for element in forth_row {
            println!("{:?}", element.value().attr("href"));
          }
        },
        Err(_) => println!("Parse text error")
      }
    },
    Err(_) => {
      println!("Get error")
    }
  }
}

#[test]
fn test_cartoonmad_manga_info() {
  let res = reqwest::get("https://cartoonmad.com/comic/6312.html");
  match res {
    Ok(mut sth) => {
      match sth.text_with_charset("big5") {
        Ok(html_txt) => {
          let document = Html::parse_document(&html_txt);
          let selector = Selector::parse("body > table > tbody > tr:first-child > td:nth-child(2) > table > tbody").unwrap();
          let ps = document.select(&selector).next().unwrap();
          let title_selector = Selector::parse("tr:nth-child(3) > td:nth-child(2) > a:last-child").unwrap();
          let title = ps.select(&title_selector).next().unwrap();
          let title_inner = title.inner_html();
          println!("A Elem: {}", title.html());
          println!("A Elem Length: {}", title.html().len());
          println!("Raw Title: {}", title_inner);
          println!("Length: {}", title_inner.len());
          println!("{:X?}", title_inner.as_bytes());
        },
        Err(_) => println!("Parse text error")
      }
    },
    Err(_) => {
      println!("Get error")
    }
  }
}