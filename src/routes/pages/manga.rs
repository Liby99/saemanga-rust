use rocket_contrib::templates::Template;
use rocket::response::Redirect;

use crate::util::{Error, Database};
use crate::app::user::User;
use crate::app::follow::Follow;
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
  scale_percentage: f32,
  main_width: f32,
}

#[derive(Serialize)]
struct PageData<'a> {
  url: String,
  user: Option<&'a User>,
  manga: &'a Manga,
  episode: EpisodeData,
  setting: SettingData,
}

impl From<UserSetting> for SettingData {
  fn from(setting: UserSetting) -> Self {
    let s : f32 = setting.scale.get();
    Self {
      is_left_hand_mode: setting.hand_mode == HandMode::Left,
      is_night_mode: setting.light_mode == LightMode::Night,
      scale_percentage: (s * 100.0).round(),
      main_width: 768.0 * s,
    }
  }
}

#[get("/manga/<dmk_id>")]
pub fn manga(user: &User, conn: Database, dmk_id: String) -> Redirect {
  match Follow::get_or_upsert(&conn, user, &dmk_id, None) {
    Ok((follow, _)) => {
      Redirect::to(format!("/manga/{}/{}", dmk_id, follow.max_episode()))
    },
    Err(err) => err.redirect(None),
  }
}

#[get("/manga/<dmk_id>", rank=2)]
pub fn manga_without_user(conn: Database, dmk_id: String) -> Redirect {
  match Manga::get_or_fetch_by_dmk_id(&conn, &dmk_id) {
    Ok(manga) => {
      let data = manga.data();
      let first_epi = data.first_episode();
      Redirect::to(data.saemanga_episode_url(first_epi.episode()))
    },
    Err(err) => err.redirect(None)
  }
}

fn render_page(
  user: Option<&User>,
  setting: UserSetting,
  manga: &Manga,
  epi: i32
) -> Result<Template, Redirect> {
  let data = manga.data();
  let episode = data.find_episode(epi).ok_or(Error::InvalidEpisode.redirect(None))?;
  let next_episode = data.next_episode_of(&episode);
  let prev_episode = data.prev_episode_of(&episode);
  Ok(Template::render("manga", PageData {
    url: data.saemanga_episode_url(episode.episode()),
    user: user,
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
  }))
}

#[get("/manga/<dmk_id>/<epi>")]
pub fn manga_with_epi(
  conn: Database,
  user: &User,
  setting: UserSetting,
  dmk_id: String,
  epi: i32,
) -> Result<Template, Redirect> {
  let (_, manga) = match Follow::get_or_upsert(&conn, user, &dmk_id, Some(epi)) {
    Ok(result) => result,
    Err(err) => return Err(err.redirect(None))
  };
  render_page(Some(user), setting, &manga, epi)
}

#[get("/manga/<dmk_id>/<epi>", rank=2)]
pub fn manga_with_epi_without_user(
  conn: Database,
  setting: UserSetting,
  dmk_id: String,
  epi: i32,
) -> Result<Template, Redirect> {
  let manga = Manga::get_or_fetch_by_dmk_id(&conn, &dmk_id).map_err(|err| err.redirect(None))?;
  render_page(None, setting, &manga, epi)
}