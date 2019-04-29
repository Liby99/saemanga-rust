use std::cmp::Ordering;

#[derive(Debug)]
pub struct Genre {
  id: &'static str,
  name: &'static str,
  dmk_directory: &'static str,
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

static GENRE_IDS_SORTED: [&'static str; 14] = [
  "adv",
  "combat",
  "comedy",
  "detective",
  "gakuen",
  "horror",
  "magen",
  "magic",
  "other",
  "scifi",
  "sengoku",
  "shojo",
  "shonen",
  "sport",
];

static GENRES_IN_ID_SORT: [&'static Genre; 14] = [
  &ADV,
  &COMBAT,
  &COMEDY,
  &DETECTIVE,
  &GAKUEN,
  &HORROR,
  &MAGEN,
  &MAGIC,
  &OTHER,
  &SCIFI,
  &SENGOKU,
  &SHOJO,
  &SHONEN,
  &SPORT,
];

impl Genre {
  pub fn for_id(id: &str) -> Option<&'static Genre> {
    let candidate = id.as_bytes();
    match GENRE_IDS_SORTED.binary_search_by(|probe| {
      let bytes = probe.as_bytes();
      let c = bytes.len().cmp(&candidate.len());
      if c != Ordering::Equal {
        return c;
      }
      let probe_iter = bytes.iter().rev();
      let candidate_iter = candidate.iter().rev();
      probe_iter.cmp(candidate_iter)
    }) {
      Ok(i) => Some(GENRES_IN_ID_SORT[i]),
      Err(_) => None,
    }
  }

  pub fn id(&self) -> String {
    self.id.to_string()
  }

  pub fn name(&self) -> String {
    self.name.to_string()
  }

  pub fn dmk_url(&self) -> String {
    format!("") // TODO
  }
}