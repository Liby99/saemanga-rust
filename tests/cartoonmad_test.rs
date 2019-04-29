use scraper::{Selector, Html};
use encoding::{EncoderTrap, DecoderTrap};
use encoding::all::BIG5_2003;
use encoding::types::Encoding;

#[test]
fn test_cartoonmad_homepage() {
  let res = reqwest::get("https://cartoonmad.com");
  match res {
    Ok(mut sth) => {
      match sth.text() {
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
          // for element in rows[4] {
          //   println!("{:?}", element.html());
          // }
          // match BIG5_2003.decode(html_txt.as_bytes(), DecoderTrap::Ignore) {
          //   Ok(decoded) => println!("{:?}", decoded),
          //   Err(_) => println!("Decode text to big5 error")
          // }
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
      match sth.text() {
        Ok(html_txt) => {
          let document = Html::parse_document(&html_txt);
          let selector = Selector::parse("body > table > tbody > tr:first-child > td:nth-child(2) > table > tbody").unwrap();
          let ps = document.select(&selector).next().unwrap();
          let title_selector = Selector::parse("tr:nth-child(3) > td:nth-child(2) > a:last-child").unwrap();
          let title = ps.select(&title_selector).next().unwrap().inner_html();
          println!("{:X?}", title.as_bytes());
          match BIG5_2003.decode(title.as_bytes(), DecoderTrap::Ignore) {
            Ok(decoded) => println!("{:?}", decoded),
            Err(_) => println!("Decode text from big5 error")
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
fn test_big5_encode() {
  let string = String::from("鬱悶飯");
  println!("{:X?}", BIG5_2003.encode(&string, EncoderTrap::Ignore));
}

#[test]
fn test_big5_decode() {
  let arr : [u8; 6] = [0xC6, 0x7B, 0xB4, 0x65, 0xB6, 0xBA];
  println!("{:X?}", BIG5_2003.decode(&arr, DecoderTrap::Ignore));
}