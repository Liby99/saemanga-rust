use scraper::{Selector, Html};

use crate::app::manga::*;
use crate::app::genre::*;

pub fn fetch_manga_data(dmk_id: &String) -> Result<(), String> {

  // Check validity of dmk_id
  assert!(is_valid_dmk_id(dmk_id), format!("Invalid dmk_id {}", dmk_id));

  // Generate url and make the request
  let url = format!("https://cartoonmad.com/comic/{}.html", dmk_id);
  let res_wrp = reqwest::get(url.as_str());

  // Check the response
  match res_wrp {
    Ok(mut res) => {
      match res.text_with_charset("big5") {
        Ok(html_text) => {
          // Now the response is turned into valid Utf-8 Html Text

          // First go to scraper parse
          let document = Html::parse_document(&html_text);
          let main_tr_selector = Selector::parse("body > table > tbody > tr:first-child > td:nth-child(2) > table > tbody").unwrap();
          let main_tbody = document.select(&main_tr_selector).next().unwrap();

          // Then extract the title information
          let title = {
            let sel = Selector::parse("tr:nth-child(3) > td:nth-child(2) > a:last-child").unwrap();
            main_tbody.select(&sel).next().unwrap().inner_html().to_string()
          };

          println!("Title: {:?}", title);

          // Then go to the info section
          let info_td_selector = Selector::parse("tr:nth-child(4) > td > table > tbody > tr:nth-child(2) > td:nth-child(2)").unwrap();
          let info_td = main_tbody.select(&info_td_selector).next().unwrap();

          // Extract these information
          let (genre, author) = {
            let info_tbody_selector = Selector::parse("table:first-child > tbody").unwrap();
            let info_tbody = info_td.select(&info_tbody_selector).next().unwrap();

            // Extract the genre information
            let genre = {
              let sel = Selector::parse("tr:nth-child(3) > td > a").unwrap();
              match info_tbody.select(&sel).next().unwrap().value().attr("href") {
                Some(href) => match Genre::from_dmk_genre_url(href) {
                  Some(genre) => genre,
                  None => {
                    return Err(format!("Cannot extract genre information from url {}", href));
                  },
                },
                None => {
                  return Err(format!("Genre information doesn't exist"));
                }
              }
            };

            // Extract the author information
            let author = {
              let sel = Selector::parse("tr:nth-child(5) > td").unwrap();
              let text = info_tbody.select(&sel).next().unwrap().text().collect::<Vec<_>>();
              let mut iter = text[0].trim().split_whitespace();
              iter.next();
              match iter.next() {
                Some(s) => s.to_string(),
                None => {
                  return Err(format!("Cannot extract author information for manga {}", dmk_id));
                }
              }
            };

            (genre, author)
          };

          println!("Genre: {:?}", genre);
          println!("Author: {:?}", author);

          Ok(())
        },
        Err(_) => Err(format!("Cannot decode html content from {}", url))
      }
    },
    Err(_) => Err(format!("Fetch manga info from {} failed", url))
  }
}