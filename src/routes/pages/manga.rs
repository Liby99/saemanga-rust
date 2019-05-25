use rocket_contrib::templates::Template;
use rocket::response::Redirect;

use crate::util::{Error, Database};
use crate::app::user::User;
use crate::app::manga::Manga;
use crate::app::manga_data::{MangaData, MangaEpisode};
use crate::app::user_setting::*;

#[derive(Serialize)]
struct NeighborEpisodeData {
  is_book: bool,
  url: String,
}

fn to_neighbor_episode_data(manga: &MangaData, epi: &MangaEpisode) -> NeighborEpisodeData {
  NeighborEpisodeData {
    is_book: epi.is_book(),
    url: manga.saemanga_episode_url(epi.episode()),
  }
}

#[derive(Serialize)]
struct EpisodeData {

  // Basic information about the current episode
  episode: i32,
  is_book: bool,
  pages: Vec<String>,

  // Neighbor episodes
  prev: Option<NeighborEpisodeData>,
  next: Option<NeighborEpisodeData>,
}

#[derive(Serialize)]
struct SettingData {
  is_left_hand_mode: bool,
  is_night_mode: bool,
  scale: f32,
}

#[derive(Serialize)]
struct PageData<'a> {
  manga: &'a Manga,
  episode: EpisodeData,
  setting: SettingData,
}

impl From<UserSetting> for SettingData {
  fn from(setting: UserSetting) -> Self {
    Self {
      is_left_hand_mode: setting.hand_mode == HandMode::Left,
      is_night_mode: setting.light_mode == LightMode::Night,
      scale: setting.scale.get(),
    }
  }
}

#[get("/manga/<dmk_id>")]
pub fn manga(
  user: Option<&User>,
  conn: Database,
  setting: UserSetting,
  dmk_id: String
) -> Redirect {
  match Manga::get_by_dmk_id(&conn, &dmk_id) {
    Ok(maybe_manga) => match maybe_manga {
      Some(manga) => {
        let data = manga.data();
        let first_epi = data.first_episode();
        Redirect::to(data.saemanga_episode_url(first_epi.episode()))
      },
      None => Error::MangaNotFoundError.redirect(None)
    },
    Err(err) => err.redirect(None)
  }
}

#[get("/manga/<dmk_id>/<epi>")]
pub fn manga_with_epi(
  user: Option<&User>,
  conn: Database,
  setting: UserSetting,
  dmk_id: String,
  epi: i32,
) -> Template {
  let manga = Manga::get_by_dmk_id(&conn, &dmk_id).unwrap().unwrap();
  let data = &manga.data();
  let episode = data.find_episode(epi).unwrap();
  let next_episode = data.next_episode_of(&episode);
  let prev_episode = data.prev_episode_of(&episode);

  Template::render("manga", PageData {
    manga: &manga,
    episode: EpisodeData {
      episode: episode.episode(),
      is_book: episode.is_book(),
      pages: (std::ops::Range { start: 1, end: episode.num_pages() + 1 }).map(|page| {
        data.dmk_image_url(episode.episode(), page)
      }).collect::<Vec<_>>(),
      next: next_episode.map(|epi| to_neighbor_episode_data(data, epi)),
      prev: prev_episode.map(|epi| to_neighbor_episode_data(data, epi)),
    },
    setting: SettingData::from(setting),
  })
}