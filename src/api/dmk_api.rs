use lazy_static::lazy_static;
use regex::Regex;
use scraper::{Selector, Html};
use scraper::element_ref::{ElementRef, Select};
use reqwest::RedirectPolicy;
use reqwest::header::{CONTENT_TYPE, LOCATION, REFERER};
use std::option::NoneError;
use encoding::all::BIG5_2003;
use encoding::{EncoderTrap};
use encoding::types::Encoding;

use crate::app::manga::*;
use crate::app::genre::*;

#[derive(Debug)]
pub struct DmkScrapeError(String);

impl DmkScrapeError {
  fn new(msg: String) -> Self {
    DmkScrapeError(msg)
  }
}

impl std::fmt::Display for DmkScrapeError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Scraper Error: {}", self.0)
  }
}

impl From<NoneError> for DmkScrapeError {
  fn from(error: NoneError) -> Self {
    Self::new(format!("Unwrapping none error: {:?}", error))
  }
}

impl From<reqwest::Error> for DmkScrapeError {
  fn from(error: reqwest::Error) -> Self {
    Self::new(format!("Reqwesting webpage error: {:?}", error))
  }
}

fn extract_num_pages(font: &String) -> Option<u32> {
  lazy_static! { static ref NUM_PAGES_RE : Regex = Regex::new(r"(\d+)").unwrap(); }
  match NUM_PAGES_RE.captures(font) {
    Some(cap) => match String::from(&cap[1]).parse::<u32>() { Ok(i) => Some(i), Err(_) => None },
    None => None,
  }
}

fn extract_episode(a: &String) -> Option<(u32, bool)> {
  lazy_static! { static ref EPISODE_RE : Regex = Regex::new(r"(\d+)").unwrap(); }
  match EPISODE_RE.captures(a) {
    Some(cap) => match String::from(&cap[1]).parse::<u32>() { Ok(i) => Some((i, a.contains("卷"))), Err(_) => None },
    None => None
  }
}

fn extract_episodes(trs: Select, start_index: usize) -> Vec<MangaEpisode> {
  lazy_static!{
    static ref TD_SEL : Selector = Selector::parse("td").unwrap();
    static ref A_SEL : Selector = Selector::parse("a").unwrap();
    static ref FONT_SEL : Selector = Selector::parse("font").unwrap();
  }
  let mut index_count = start_index;
  trs.map(|tr: ElementRef| {
    tr.select(&TD_SEL).filter_map(|td: ElementRef| {
      match td.select(&A_SEL).next() {
        Some(a_elem) => {
          let index = index_count;
          let (episode, is_book) = extract_episode(&a_elem.inner_html()).unwrap();
          let num_pages = extract_num_pages(&td.select(&FONT_SEL).next()?.inner_html()).unwrap();
          let href = a_elem.value().attr("href").unwrap().to_string();
          index_count += 1;
          Some(MangaEpisode::new(index, is_book, episode, num_pages, href))
        }, None => None
      }
    }).collect::<Vec<_>>()
  }).flatten().collect::<Vec<_>>()
}

pub fn fetch_manga_data(dmk_id: &String) -> Result<Manga, DmkScrapeError> {

  // Check validity of dmk_id
  assert!(is_valid_dmk_id(dmk_id), format!("Invalid dmk_id {}", dmk_id));

  // Generate url and make the request
  let url = format!("https://cartoonmad.com/comic/{}.html", dmk_id);
  let html_text = reqwest::get(url.as_str())?.text_with_charset("big5")?;

  // First go to scraper parse
  let document = Html::parse_document(&html_text);
  lazy_static!{
    static ref MAIN_TR_SEL : Selector = Selector::parse(
      "body > table > tbody > tr:first-child > td:nth-child(2) > table > tbody"
    ).unwrap();
  }
  let main_tbody = document.select(&MAIN_TR_SEL).next()?;

  // Then extract the title information
  let title = {
    lazy_static!{ static ref TITLE_SEL : Selector = Selector::parse("tr:nth-child(3) > td:nth-child(2) > a:last-child").unwrap(); }
    main_tbody.select(&TITLE_SEL).next()?.inner_html().to_string()
  };

  // Then go to the info section
  lazy_static!{
    static ref INFO_TD_SEL : Selector = Selector::parse(
      "tr:nth-child(4) > td > table > tbody > tr:nth-child(2) > td:nth-child(2)"
    ).unwrap();
  }
  let info_td = main_tbody.select(&INFO_TD_SEL).next()?;

  // Extract these information
  let (genre, author, tags, status) = {
    lazy_static!{ static ref INFO_TBODY_SEL : Selector = Selector::parse("table:first-child > tbody").unwrap(); }
    let info_tbody = info_td.select(&INFO_TBODY_SEL).next()?;

    // Extract the genre information
    let genre : &'static Genre = {
      lazy_static!{ static ref GENRE_SEL : Selector = Selector::parse("tr:nth-child(3) > td > a").unwrap(); }
      match info_tbody.select(&GENRE_SEL).next()?.value().attr("href") {
        Some(href) => match Genre::from_dmk_genre_url(href) {
          Some(genre) => genre,
          None => {
            return Err(DmkScrapeError::new(format!("Cannot extract genre information from url {}", href)));
          },
        },
        None => { return Err(DmkScrapeError::new(format!("Genre information doesn't exist"))); }
      }
    };

    // Extract the author information
    let author : String = {
      lazy_static!{ static ref AUTHOR_SEL : Selector = Selector::parse("tr:nth-child(5) > td").unwrap(); }
      let text = info_tbody.select(&AUTHOR_SEL).next()?.text().collect::<Vec<_>>();
      let mut iter = text[0].trim().split_whitespace();
      iter.next();
      String::from(iter.next()?)
    };

    // Extract tag information
    let tags : Vec<String> = {
      lazy_static!{ static ref TAGS_SEL : Selector = Selector::parse("tr:nth-child(14) > td > a").unwrap(); }
      info_tbody.select(&TAGS_SEL).map(|e| String::from(e.inner_html())).collect::<Vec<_>>()
    };

    // Extract ended information
    let status : MangaStatus = {

      // Make the selector lazy static
      lazy_static!{
        static ref STATUS_IMG_SEL : Selector = Selector::parse(
          "tr:nth-child(7) > td > img:last-child"
        ).unwrap();
      }

      // Get the status
      let img_src = info_tbody.select(&STATUS_IMG_SEL).next()?.value().attr("src")?;
      match img_src.find('9') {
        Some(_) => MangaStatus::Ended,
        None => MangaStatus::Updating
      }
    };

    // Get back the basic informations
    (genre, author, tags, status)
  };

  // Extract descriptions
  let description : String = {

    // Make the selector lazy static
    lazy_static!{
      static ref DESCRIPTION_SEL : Selector = Selector::parse(
        "table:nth-child(2) > tbody > tr > td > fieldset > table > tbody > tr > td"
      ).unwrap();
    }

    // Get the description
    info_td.select(&DESCRIPTION_SEL).next()?.text().collect::<Vec<_>>().join(" ").trim().to_string()
  };

  // Extract book and episode information
  let episodes : Vec<MangaEpisode> = {

    // Make the selector lazy static
    lazy_static!{
      static ref TABLE_SEL : Selector = Selector::parse(
        "table:nth-child(3) > tbody > tr > td > fieldset > table"
      ).unwrap();
      static ref TR_SEL : Selector = Selector::parse("tbody > tr").unwrap();
    }

    // First get all tables
    let mut tables = info_td.select(&TABLE_SEL);

    // Get the first table and extract it to episodes
    let first_table = tables.next()?;
    match tables.next() {
      Some(second_table) => {
        let books = extract_episodes(first_table.select(&TR_SEL), 0);
        let episodes = extract_episodes(second_table.select(&TR_SEL), books.len());
        [&books[..], &episodes[..]].concat()
      },
      None => extract_episodes(first_table.select(&TR_SEL), 0)
    }
  };

  // Get the image information
  let dmk_id_base : DmkIdBase = {

    // Get episode webpage
    let epi_url = format!("https://www.cartoonmad.com{}", &episodes[0].dmk_directory());
    let epi_html_text = reqwest::get(epi_url.as_str())?.text_with_charset("big5")?;

    // Make the selector lazy static
    lazy_static!{
      static ref IMG_SEL : Selector = Selector::parse(
        "body > table > tbody > tr:nth-child(5) > td > table > tbody > tr:first-child > td:first-child > a > img"
      ).unwrap();
    }

    // Parse the episode html text to dom
    let epi_document = Html::parse_document(epi_html_text.as_str());
    let img_elem = epi_document.select(&IMG_SEL).next()?;
    let img_src = img_elem.value().attr("src")?;
    let full_img_url = format!("https://www.cartoonmad.com/comic/{}", img_src);

    // Request the image url and get the response header
    let client = reqwest::Client::builder().redirect(RedirectPolicy::none()).build()?;
    let res = client.get(full_img_url.as_str()).header(REFERER, epi_url).send()?;
    match res.headers().get(LOCATION)?.to_str() {
      Ok(s) => {
        // Finally convert the true location into dmk id base
        let true_img_loc = String::from(s);
        DmkIdBase::from_dmk_image_url(&true_img_loc)?
      },
      Err(_) => {
        return Err(DmkScrapeError::new(String::from("Not able to fetch real image location")))
      }
    }
  };

  // Return the finally extracted Manga object
  Ok(Manga::new(dmk_id.clone(), dmk_id_base, title, description, author, tags, genre, status, episodes))
}

fn get_manga_ids_from_a_elems<'a>(a_elems: impl Iterator<Item=ElementRef<'a>>) -> Result<Vec<String>, DmkScrapeError> {
  let ids = a_elems.filter_map(|a: ElementRef| {
    match a.value().attr("href") {
      Some(href) => {
        lazy_static! { static ref COMIC_URL_REG : Regex = Regex::new(r"comic/(\d+).html").unwrap(); }
        match COMIC_URL_REG.captures(href) {
          Some(cap) => Some(String::from(&cap[1])),
          None => None
        }
      },
      None => None
    }
  }).collect::<Vec<_>>();
  Ok(ids)
}

lazy_static! {
  static ref MANGA_INDEX_A_SEL : Selector = Selector::parse(
    "body > table > tbody > tr:first-child > \
    td:nth-child(2) > table > tbody > tr:nth-child(4) > td > table > tbody > tr:nth-child(2) > \
    td:nth-child(2) > table > tbody > tr > td > table > tbody > tr > td > a"
  ).unwrap();
}

fn fetch_latest_manga_with_url(url: &String) -> Result<Vec<String>, DmkScrapeError> {
  let html_text = reqwest::get(url.as_str())?.text_with_charset("big5")?;
  let document = Html::parse_document(&html_text);

  // Select the a elements that the hrefs include the dmk_id
  let a_elems = document.select(&MANGA_INDEX_A_SEL);
  Ok(get_manga_ids_from_a_elems(a_elems)?)
}

pub fn fetch_latest_manga() -> Result<Vec<String>, DmkScrapeError> {
  fetch_latest_manga_with_url(&String::from("https://cartoonmad.com/"))
}

pub fn fetch_latest_manga_with_genre(genre: &'static Genre) -> Result<Vec<String>, DmkScrapeError> {
  fetch_latest_manga_with_url(&genre.dmk_url())
}

pub fn search(text: &String) -> Result<Vec<String>, DmkScrapeError> {

  // First generate the keyword. We are assuming the given text is already traditional chinese
  let keyword : String = {
    match BIG5_2003.encode(text, EncoderTrap::Ignore) {
      Ok(byte_arr) => byte_arr.into_iter().map(|b: u8| format!("%{:X}", b)).collect::<Vec<_>>().join(""),
      Err(_) => { return Err(DmkScrapeError(String::from("Keyword encoding error"))); }
    }
  };

  // Make the post request for search and get the html text
  let client = reqwest::Client::new();
  let query = format!("keyword={}&searchtype=all", keyword);
  let mut response = client.post("https://cartoonmad.com/search.html")
    .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
    .body(query.into_bytes()) // Has to send the body using raw bytes array
    .send()?;
  let html_text = response.text_with_charset("big5")?;

  // Parse the documents and get the A elements
  let document = Html::parse_document(&html_text);
  let a_elems = document.select(&MANGA_INDEX_A_SEL);

  // Return the dmk_ids array
  Ok(get_manga_ids_from_a_elems(a_elems)?)
}