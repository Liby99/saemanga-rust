use lazy_static::lazy_static;
use regex::Regex;
use scraper::{Selector, Html};
use scraper::element_ref::{ElementRef, Select};
use reqwest::RedirectPolicy;
use reqwest::header::{LOCATION, REFERER};
use std::option::NoneError;

use crate::app::manga::*;
use crate::app::genre::*;

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
  let td_sel = Selector::parse("td").unwrap();
  let a_sel = Selector::parse("a").unwrap();
  let font_sel = Selector::parse("font").unwrap();
  let mut index_count = start_index;
  trs.map(|tr: ElementRef| {
    tr.select(&td_sel).filter_map(|td: ElementRef| {
      match td.select(&a_sel).next() {
        Some(a_elem) => {
          let index = index_count;
          let (episode, is_book) = extract_episode(&a_elem.inner_html()).unwrap();
          let num_pages = extract_num_pages(&td.select(&font_sel).next()?.inner_html()).unwrap();
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
  let main_tr_selector = Selector::parse("body > table > tbody > tr:first-child > td:nth-child(2) > table > tbody").unwrap();
  let main_tbody = document.select(&main_tr_selector).next()?;

  // Then extract the title information
  let title = {
    let sel = Selector::parse("tr:nth-child(3) > td:nth-child(2) > a:last-child").unwrap();
    main_tbody.select(&sel).next()?.inner_html().to_string()
  };

  // Then go to the info section
  let info_td_selector = Selector::parse("tr:nth-child(4) > td > table > tbody > tr:nth-child(2) > td:nth-child(2)").unwrap();
  let info_td = main_tbody.select(&info_td_selector).next()?;

  // Extract these information
  let (genre, author, tags, status) = {
    let info_tbody_selector = Selector::parse("table:first-child > tbody").unwrap();
    let info_tbody = info_td.select(&info_tbody_selector).next()?;

    // Extract the genre information
    let genre : &'static Genre = {
      let sel = Selector::parse("tr:nth-child(3) > td > a").unwrap();
      match info_tbody.select(&sel).next()?.value().attr("href") {
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
      let sel = Selector::parse("tr:nth-child(5) > td").unwrap();
      let text = info_tbody.select(&sel).next()?.text().collect::<Vec<_>>();
      let mut iter = text[0].trim().split_whitespace();
      iter.next();
      String::from(iter.next()?)
    };

    // Extract tag information
    let tags : Vec<String> = {
      let sel = Selector::parse("tr:nth-child(14) > td > a").unwrap();
      info_tbody.select(&sel).map(|e| String::from(e.inner_html())).collect::<Vec<_>>()
    };

    // Extract ended information
    let status : MangaStatus = {
      let sel = Selector::parse("tr:nth-child(7) > td > img:last-child").unwrap();
      let img_src = info_tbody.select(&sel).next()?.value().attr("src")?;
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
    let sel = Selector::parse("table:nth-child(2) > tbody > tr > td > fieldset > table > tbody > tr > td").unwrap();
    info_td.select(&sel).next()?.text().collect::<Vec<_>>().join(" ").trim().to_string()
  };

  // Extract book and episode information
  let episodes : Vec<MangaEpisode> = {

    // First get all tables
    let table_sel = Selector::parse("table:nth-child(3) > tbody > tr > td > fieldset > table").unwrap();
    let tr_sel = Selector::parse("tbody > tr").unwrap();
    let mut tables = info_td.select(&table_sel);

    // Get the first table and extract it to episodes
    let first_table = tables.next()?;
    match tables.next() {
      Some(second_table) => {
        let books = extract_episodes(first_table.select(&tr_sel), 0);
        let episodes = extract_episodes(second_table.select(&tr_sel), books.len());
        [&books[..], &episodes[..]].concat()
      },
      None => extract_episodes(first_table.select(&tr_sel), 0)
    }
  };

  // Get the image information
  let dmk_id_base : DmkIdBase = {

    // Get episode webpage
    let epi_url = format!("https://www.cartoonmad.com{}", &episodes[0].dmk_directory());
    let epi_html_text = reqwest::get(epi_url.as_str())?.text_with_charset("big5")?;

    // Parse the episode html text to dom
    let epi_document = Html::parse_document(epi_html_text.as_str());
    let img_sel = Selector::parse("body > table > tbody > tr:nth-child(5) > td > table > tbody > tr:first-child > td:first-child > a > img").unwrap();
    let img_elem = epi_document.select(&img_sel).next()?;
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

fn get_latest_manga_from_row(a_elems: Select) -> Result<Vec<String>, DmkScrapeError> {
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

fn fetch_latest_manga_with_url(url: &String) -> Result<Vec<String>, DmkScrapeError> {
  let html_text = reqwest::get(url.as_str())?.text_with_charset("big5")?;
  let document = Html::parse_document(&html_text);

  // Select the tbody that includes the latest mangas
  let tbody_sel = Selector::parse("body > table > tbody > tr:first-child > td:nth-child(2) > table > tbody > tr:nth-child(4) > td > table > tbody > tr:nth-child(2) > td:nth-child(2) > table > tbody").unwrap();
  let tbody = document.select(&tbody_sel).next()?;

  // Get the manga rows
  let first_row_sel = Selector::parse("tr:nth-child(3) > td > a").unwrap();
  let second_row_sel = Selector::parse("tr:nth-child(5) > td > a").unwrap();

  let first_row_ids = get_latest_manga_from_row(tbody.select(&first_row_sel))?;
  let second_row_ids = get_latest_manga_from_row(tbody.select(&second_row_sel))?;

  Ok([first_row_ids, second_row_ids].concat())
}

pub fn fetch_latest_manga() -> Result<Vec<String>, DmkScrapeError> {
  fetch_latest_manga_with_url(&String::from("https://cartoonmad.com/"))
}

pub fn fetch_latest_manga_with_genre(genre: &'static Genre) -> Result<Vec<String>, DmkScrapeError> {
  fetch_latest_manga_with_url(&genre.dmk_url())
}