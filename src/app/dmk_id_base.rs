use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum DmkIdBase {
  V10 { dmk_id_web: String, dmk_id_home: String },
  V09 { dmk_id_home: String },
  V08 { dmk_id_home: String },
  V07 { dmk_id_gen: String },
  V06 { dmk_id_gen: String },
  V05 { dmk_id_web: String, dmk_id_gen: String },
}

fn parse_v10_image_url(url: &String) -> Option<DmkIdBase> {
  lazy_static! { static ref IMG_RE_10 : Regex = Regex::new(r"(web\d+)\.cartoonmad\.com/(home\d+)/(\d+)/\d+/\d+\.jpg").unwrap(); }
  match IMG_RE_10.captures(url.as_str()) {
    Some(cap) => Some(DmkIdBase::V10 { dmk_id_web: cap[1].to_string(), dmk_id_home: cap[2].to_string() }),
    None => None
  }
}

fn parse_v09_image_url(url: &String) -> Option<DmkIdBase> {
  lazy_static! { static ref IMG_RE_09 : Regex = Regex::new(r"(home\d+)/(\d+)/\d+/\d+\.jpg").unwrap(); }
  match IMG_RE_09.captures(url.as_str()) {
    Some(cap) => Some(DmkIdBase::V09 { dmk_id_home: cap[1].to_string() }),
    None => None
  }
}

fn parse_v08_image_url(url: &String) -> Option<DmkIdBase> {
  lazy_static! { static ref IMG_RE_08 : Regex = Regex::new(r"^/([\w\d]+)/(\d+)/\d+/\d+\.jpg$").unwrap(); }
  match IMG_RE_08.captures(url.as_str()) {
    Some(cap) => Some(DmkIdBase::V08 { dmk_id_home: cap[1].to_string() }),
    None => None
  }
}

fn parse_v07_image_url(url: &String) -> Option<DmkIdBase> {
  lazy_static! { static ref IMG_RE_07 : Regex = Regex::new(r"^/home1/([\d\w]+)/(\d+)/\d+/\d+\.jpg$").unwrap(); }
  match IMG_RE_07.captures(url.as_str()) {
    Some(cap) => Some(DmkIdBase::V07 { dmk_id_gen: cap[1].to_string() }),
    None => None
  }
}

fn parse_v06_image_url(url: &String) -> Option<DmkIdBase> {
  lazy_static! { static ref IMG_RE_06 : Regex = Regex::new(r"^/cartoonimg/([\d\w]+)/(\d+)/\d+/\d+\.jpg$").unwrap(); }
  match IMG_RE_06.captures(url.as_str()) {
    Some(cap) => Some(DmkIdBase::V06 { dmk_id_gen: cap[1].to_string() }),
    None => None
  }
}

fn parse_v05_image_url(url: &String) -> Option<DmkIdBase> {
  lazy_static! { static ref IMG_RE_05 : Regex = Regex::new(r"^https?://(web\d?)\.cartoonmad\.com/([\w|\d]+)/").unwrap(); }
  match IMG_RE_05.captures(url.as_str()) {
    Some(cap) => Some(DmkIdBase::V05 { dmk_id_web: cap[1].to_string(), dmk_id_gen: cap[2].to_string() }),
    None => None
  }
}

impl DmkIdBase {
  pub fn from_dmk_image_url(url: &String) -> Option<Self> {
    let functions : Vec<&Fn(&String) -> Option<Self>> = vec![
      &parse_v10_image_url, &parse_v09_image_url, &parse_v08_image_url,
      &parse_v07_image_url, &parse_v06_image_url, &parse_v05_image_url,
    ];
    for f in functions {
      match f(url) { Some(res) => { return Some(res); }, _ => () }
    }
    None
  }

  pub fn dmk_image_url_base(&self) -> String {
    match self {
      DmkIdBase::V10 { dmk_id_web, dmk_id_home } => format!("http://{}.cartoonmad.com/{}", dmk_id_web, dmk_id_home),
      DmkIdBase::V09 { dmk_id_home } => format!("http://cartoonmad.com/{}", dmk_id_home),
      DmkIdBase::V08 { dmk_id_home } => format!("http://cartoonmad.com/{}", dmk_id_home),
      DmkIdBase::V07 { dmk_id_gen } => format!("http://www.cartoonmad.com/home1/{}", dmk_id_gen),
      DmkIdBase::V06 { dmk_id_gen } => format!("http://www.cartoonmad.com/cartoonimg/{}", dmk_id_gen),
      DmkIdBase::V05 { dmk_id_web, dmk_id_gen } => format!("http://{}.cartoonmad.com/{}", dmk_id_web, dmk_id_gen),
    }
  }
}