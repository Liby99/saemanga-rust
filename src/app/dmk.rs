use std::thread;

use lazy_static::lazy_static;
use regex::Regex;
use scraper::{Selector, Html};
use scraper::element_ref::{ElementRef, Select};
use reqwest::RedirectPolicy;
use reqwest::header::{CONTENT_TYPE, LOCATION, REFERER};
use encoding::all::BIG5_2003;
use encoding::{EncoderTrap};
use encoding::types::Encoding;

use crate::app::manga_data::*;
use crate::app::genre::*;
use crate::util::Error;

fn extract_num_pages(font: &String) -> Option<i32> {
  lazy_static! { static ref NUM_PAGES_RE : Regex = Regex::new(r"(\d+)").unwrap(); }
  match NUM_PAGES_RE.captures(font) {
    Some(cap) => match String::from(&cap[1]).parse::<i32>() {
      Ok(i) => Some(i),
      Err(_) => None
    },
    None => None,
  }
}

fn extract_episode(a: &String) -> Option<(i32, bool)> {
  lazy_static! { static ref EPISODE_RE : Regex = Regex::new(r"(\d+)").unwrap(); }
  match EPISODE_RE.captures(a) {
    Some(cap) => match String::from(&cap[1]).parse::<i32>() {
      Ok(i) => Some((i, a.contains("卷"))),
      Err(_) => None
    },
    None => None
  }
}

fn extract_episodes(trs: Select, start_index: i32) -> Vec<MangaEpisode> {
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

pub fn fetch_manga_data(dmk_id: &String) -> Result<MangaData, Error> {

  // Check validity of dmk_id
  assert!(is_valid_dmk_id(dmk_id), format!("Invalid dmk_id {}", dmk_id));

  // Generate url and make the request
  let url = format!("https://cartoonmad.com/comic/{}.html", dmk_id);
  let mut response = reqwest::get(url.as_str()).map_err(|_| Error::DmkFetchError)?;
  let html_text = response.text_with_charset("big5").map_err(|_| Error::DmkEncodingError)?;

  // First go to scraper parse
  let document = Html::parse_document(&html_text);
  lazy_static!{
    static ref MAIN_TR_SEL : Selector = Selector::parse(
      "body > table > tbody > tr:first-child > td:nth-child(2) > table > tbody"
    ).unwrap();
  }
  let main_tbody = document.select(&MAIN_TR_SEL).next().ok_or(Error::DmkDomTraverseError)?;

  // Then extract the title information
  let title = {
    lazy_static!{ static ref TITLE_SEL : Selector = Selector::parse("tr:nth-child(3) > td:nth-child(2) > a:last-child").unwrap(); }
    main_tbody.select(&TITLE_SEL).next().ok_or(Error::DmkDomTraverseError)?.inner_html().to_string()
  };

  // Then go to the info section
  lazy_static!{
    static ref INFO_TD_SEL : Selector = Selector::parse(
      "tr:nth-child(4) > td > table > tbody > tr:nth-child(2) > td:nth-child(2)"
    ).unwrap();
  }
  let info_td = main_tbody.select(&INFO_TD_SEL).next().ok_or(Error::DmkDomTraverseError)?;

  // Extract these information
  let (genre, author, tags, status) = {
    lazy_static!{ static ref INFO_TBODY_SEL : Selector = Selector::parse("table:first-child > tbody").unwrap(); }
    let info_tbody = info_td.select(&INFO_TBODY_SEL).next().ok_or(Error::DmkDomTraverseError)?;

    // Extract the genre information
    let genre : &'static Genre = {
      lazy_static!{ static ref GENRE_SEL : Selector = Selector::parse("tr:nth-child(3) > td > a").unwrap(); }
      match info_tbody.select(&GENRE_SEL).next().ok_or(Error::DmkDomTraverseError)?.value().attr("href") {
        Some(href) => Genre::from_dmk_genre_url(href).ok_or(Error::GenreNotFoundError)?,
        None => return Err(Error::GenreInfoExtractionError)
      }
    };

    // Extract the author information
    let author : String = {
      lazy_static!{ static ref AUTHOR_SEL : Selector = Selector::parse("tr:nth-child(5) > td").unwrap(); }
      let text = info_tbody.select(&AUTHOR_SEL).next().ok_or(Error::DmkDomTraverseError)?.text().collect::<Vec<_>>();
      String::from(text[0].trim().split_whitespace().nth(1).ok_or(Error::DmkParseError)?)
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
      let img_elem = info_tbody.select(&STATUS_IMG_SEL).next().ok_or(Error::DmkDomTraverseError)?;
      let img_src = img_elem.value().attr("src").ok_or(Error::DmkParseError)?;
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
    let desc_elem = info_td.select(&DESCRIPTION_SEL).next().ok_or(Error::DmkDomTraverseError)?;
    desc_elem.text().collect::<Vec<_>>().join(" ").trim().to_string()
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
    let first_table = tables.next().ok_or(Error::DmkDomTraverseError)?;
    match tables.next() {
      Some(second_table) => {
        let books = extract_episodes(first_table.select(&TR_SEL), 0);
        let episodes = extract_episodes(second_table.select(&TR_SEL), books.len() as i32);
        [&books[..], &episodes[..]].concat()
      },
      None => extract_episodes(first_table.select(&TR_SEL), 0)
    }
  };

  // Get the image information
  let dmk_id_base : DmkIdBase = {

    // Get episode webpage
    let epi_url = format!("https://www.cartoonmad.com{}", &episodes[0].dmk_directory());
    let mut response = reqwest::get(epi_url.as_str()).map_err(|_| Error::DmkFetchError)?;
    let epi_html_text = response.text_with_charset("big5").map_err(|_| Error::DmkEncodingError)?;

    // Make the selector lazy static
    lazy_static!{
      static ref IMG_SEL : Selector = Selector::parse(
        "body > table > tbody > tr:nth-child(5) > td > table > tbody > tr:first-child > td:first-child > a > img"
      ).unwrap();
    }

    // Parse the episode html text to dom
    let epi_document = Html::parse_document(epi_html_text.as_str());
    let img_elem = epi_document.select(&IMG_SEL).next().ok_or(Error::DmkDomTraverseError)?;
    let img_src = img_elem.value().attr("src").ok_or(Error::DmkParseError)?;
    let full_img_url = format!("https://www.cartoonmad.com/comic/{}", img_src);

    // Request the image url and get the response header
    let client = reqwest::Client::builder().redirect(RedirectPolicy::none()).build().unwrap(); // Using unwrap because this should be done deterministically
    let res = client.get(full_img_url.as_str()).header(REFERER, epi_url).send().map_err(|_| Error::DmkFetchError)?;
    let redirect = res.headers().get(LOCATION).ok_or(Error::DmkRedirectNotFoundError)?;
    let true_img_loc = redirect.to_str().map_err(|_| Error::DmkParseError)?.to_string();
    DmkIdBase::from_dmk_image_url(&true_img_loc)?
  };

  // Return the finally extracted Manga object
  Ok(MangaData::new(dmk_id.clone(), dmk_id_base, title, description, author, tags, genre, status, episodes))
}

fn get_manga_ids_from_a_elems<'a>(a_elems: impl Iterator<Item=ElementRef<'a>>) -> Result<Vec<String>, Error> {
  Ok(a_elems.filter_map(|a: ElementRef| {
    a.value().attr("href").and_then(|href| {
      lazy_static! { static ref COMIC_URL_REG : Regex = Regex::new(r"comic/(\d+).html").unwrap(); }
      COMIC_URL_REG.captures(href).map(|cap| String::from(&cap[1]))
    })
  }).collect::<Vec<String>>())
}

lazy_static! {
  static ref MANGA_INDEX_A_SEL : Selector = Selector::parse(
    "body > table > tbody > tr:first-child > \
    td:nth-child(2) > table > tbody > tr:nth-child(4) > td > table > tbody > tr:nth-child(2) > \
    td:nth-child(2) > table > tbody > tr > td > table > tbody > tr > td > a"
  ).unwrap();
}

fn fetch_manga_ids_with_url(url: &String) -> Result<Vec<String>, Error> {
  let mut response = reqwest::get(url.as_str()).map_err(|_| Error::DmkFetchError)?;
  let html_text = response.text_with_charset("big5").map_err(|_| Error::DmkEncodingError)?;
  let document = Html::parse_document(&html_text);

  // Select the a elements that the hrefs include the dmk_id
  let a_elems = document.select(&MANGA_INDEX_A_SEL);
  Ok(get_manga_ids_from_a_elems(a_elems)?)
}

pub fn fetch_latest_manga() -> Result<Vec<String>, Error> {
  fetch_manga_ids_with_url(&String::from("https://cartoonmad.com/"))
}

pub fn fetch_latest_manga_with_genre(genre: &'static Genre) -> Result<Vec<String>, Error> {
  fetch_manga_ids_with_url(&genre.dmk_url())
}

fn dmk_ended_index_url(page: i32) -> String {
  format!("https://www.cartoonmad.com/endcm.{:02}.html", page)
}

pub fn fetch_ended() -> Result<Vec<String>, Error> {

  // Static selectors for this page
  lazy_static! {
    static ref LAST_PAGINATION_SEL : Selector = Selector::parse(
      "body > table > tbody > tr:first-child > td:nth-child(2) > table > tbody > tr:nth-child(3) > \
      td > table:nth-child(3) > tbody > tr > td:nth-child(2) > a:last-child"
    ).unwrap();
    static ref ENDED_INDEX_A_SEL : Selector = Selector::parse(
      "body > table > tbody > tr:first-child > \
      td:nth-child(2) > table > tbody > tr:nth-child(3) > td > table > tbody > tr:nth-child(2) > \
      td:nth-child(2) > table > tbody > tr > td > table > tbody > tr > td > a"
    ).unwrap();
  }

  // First get the first page data
  let first_page_url = dmk_ended_index_url(1);
  let mut response = reqwest::get(first_page_url.as_str()).map_err(|_| Error::DmkFetchError)?;
  let html_text = response.text_with_charset("big5").map_err(|_| Error::DmkEncodingError)?;
  let document = Html::parse_document(&html_text);

  // Then get the data on the first page
  let a_elems = document.select(&ENDED_INDEX_A_SEL);
  let first_page_ids = get_manga_ids_from_a_elems(a_elems)?;

  // Then get the last page count
  let pagi_elem = document.select(&LAST_PAGINATION_SEL).next().ok_or(Error::DmkDomTraverseError)?;
  let pagi_text = pagi_elem.text().next().ok_or(Error::DmkParseError)?;
  let last_page : i32 = pagi_text.parse().map_err(|_| Error::DmkParseError)?;

  type FetchHandle = thread::JoinHandle<Result<Vec<String>, Error>>;

  // For each page, create a new thread to fetch
  let handles : Vec<FetchHandle> = (2..last_page).into_iter().map(|page| {
    let url = dmk_ended_index_url(page);
    thread::spawn(move || {
      let mut response = reqwest::get(url.as_str()).map_err(|_| Error::DmkFetchError)?;
      let html_text = response.text_with_charset("big5").map_err(|_| Error::DmkEncodingError)?;
      let document = Html::parse_document(&html_text);
      let a_elems = document.select(&ENDED_INDEX_A_SEL);
      Ok(get_manga_ids_from_a_elems(a_elems)?)
    })
  }).collect();

  let ids : Vec<String> = handles.into_iter().filter_map(|handle| -> Option<Vec<String>> {
    let result = handle.join().ok()?;
    match result {
      Ok(ids) => Some(ids),
      Err(err) => {
        println!("Error when fetching ended ids {}: {}", err.code(), err.msg());
        None
      }
    }
  }).flatten().collect();

  Ok([&first_page_ids[..], &ids[..]].concat())
}

pub fn search(text: &String) -> Result<Vec<String>, Error> {

  // First generate the keyword. We are assuming the given text is already traditional chinese
  let keyword : String = {
    match BIG5_2003.encode(text, EncoderTrap::Ignore) {
      Ok(byte_arr) => byte_arr.into_iter().map(|b: u8| format!("%{:X}", b)).collect::<Vec<_>>().join(""),
      Err(_) => { return Err(Error::DmkSearchEncodingError); }
    }
  };

  // Make the post request for search and get the html text
  let client = reqwest::Client::new();
  let query = format!("keyword={}&searchtype=all", keyword);
  let mut response = client.post("https://cartoonmad.com/search.html")
    .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
    .body(query.into_bytes()) // Has to send the body using raw bytes array
    .send().map_err(|_| Error::DmkFetchError)?;
  let html_text = response.text_with_charset("big5").map_err(|_| Error::DmkEncodingError)?;

  // Parse the documents and get the A elements
  let document = Html::parse_document(&html_text);
  let a_elems = document.select(&MANGA_INDEX_A_SEL);

  // Return the dmk_ids array
  Ok(get_manga_ids_from_a_elems(a_elems)?)
}