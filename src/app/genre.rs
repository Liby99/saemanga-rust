use serde::ser::{Serialize, Serializer};
use serde::de::{self, Deserialize, Deserializer};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Genre {
  pub id: &'static str,
  pub name: &'static str,
  pub dmk_directory: &'static str,
}

pub static COMBAT : Genre = Genre {
  id: "combat",
  name: "格鬥",
  dmk_directory: "comic01",
};

pub static MAGIC : Genre = Genre {
  id: "magic",
  name: "魔法",
  dmk_directory: "comic02",
};

pub static DETECTIVE : Genre = Genre {
  id: "detective",
  name: "偵探",
  dmk_directory: "comic03",
};

pub static SPORT : Genre = Genre {
  id: "sport",
  name: "競技",
  dmk_directory: "comic04",
};

pub static HORROR : Genre = Genre {
  id: "horror",
  name: "恐怖",
  dmk_directory: "comic10",
};

pub static SENGOKU : Genre = Genre {
  id: "sengoku",
  name: "戰國",
  dmk_directory: "comic07",
};

pub static MAGEN : Genre = Genre {
  id: "magen",
  name: "魔幻",
  dmk_directory: "comic08",
};

pub static ADV : Genre = Genre {
  id: "adv",
  name: "冒險",
  dmk_directory: "comic09",
};

pub static GAKUEN : Genre = Genre {
  id: "gakuen",
  name: "校園",
  dmk_directory: "comic16",
};

pub static COMEDY : Genre = Genre {
  id: "comedy",
  name: "搞笑",
  dmk_directory: "comic17",
};

pub static SHOJO : Genre = Genre {
  id: "shojo",
  name: "少女",
  dmk_directory: "comic13",
};

pub static SHONEN : Genre = Genre {
  id: "shonen",
  name: "少男",
  dmk_directory: "comic14",
};

pub static SCIFI : Genre = Genre {
  id: "scifi",
  name: "科幻",
  dmk_directory: "comic18",
};

pub static OTHER : Genre = Genre {
  id: "other",
  name: "其他",
  dmk_directory: "comic22",
};

pub static ALL_GENRES : [&'static Genre; 14] = [
  &COMBAT,
  &MAGIC,
  &DETECTIVE,
  &SPORT,
  &HORROR,
  &SENGOKU,
  &MAGEN,
  &ADV,
  &GAKUEN,
  &COMEDY,
  &SHOJO,
  &SHONEN,
  &SCIFI,
  &OTHER,
];

impl Genre {
  pub fn all() -> Vec<&'static Genre> {
    ALL_GENRES.to_vec()
  }

  pub fn for_id(id: &str) -> Option<&'static Genre> {
    for genre in ALL_GENRES.iter() {
      if genre.id == id {
        return Some(&genre);
      }
    }
    None
  }

  pub fn for_dmk_directory(dir: &str) -> Option<&'static Genre> {
    for genre in ALL_GENRES.iter() {
      if genre.dmk_directory == dir {
        return Some(&genre);
      }
    }
    None
  }

  pub fn from_dmk_genre_url(url: &str) -> Option<&'static Genre> {
    lazy_static! { static ref DMK_GENRE_RE : Regex = Regex::new(r"^/(comic\d{2}).html$").unwrap(); }
    match DMK_GENRE_RE.captures(url) {
      Some(cap) => Self::for_dmk_directory(&cap[1]),
      None => None
    }
  }

  pub fn id(&self) -> String {
    self.id.to_string()
  }

  pub fn name(&self) -> String {
    self.name.to_string()
  }

  pub fn dmk_url(&self) -> String {
    format!("https://cartoonmad.com/{}.html", self.dmk_directory)
  }
}

impl Serialize for Genre {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    serializer.serialize_str(self.id)
  }
}

impl<'de> Deserialize<'de> for &'static Genre {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match Genre::for_id(s.as_str()) {
      Some(genre) => Ok(genre),
      None => Err(de::Error::custom(format!("Unknown genre {}", s)))
    }
  }
}