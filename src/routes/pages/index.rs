use rocket_contrib::templates::Template;

use crate::util::Database;

use crate::app::manga::Manga;
use crate::app::genre::Genre;
use crate::app::user_setting::*;
use crate::app::user::User;

#[derive(Serialize)]
struct TemplateData {
  latests: Vec<MangaData>,
  user: Option<UserData>,
  genres: Vec<GenreData>,
  setting: SettingData,
}

#[derive(Serialize)]
struct SettingData {
  is_left_hand_mode: bool,
  is_night_mode: bool,
  is_loved_only: bool,
  scale: f32,
}

impl From<UserSetting> for SettingData {
  fn from(setting: UserSetting) -> Self {
    Self {
      is_left_hand_mode: setting.hand_mode == HandMode::Left,
      is_night_mode: setting.light_mode == LightMode::Night,
      is_loved_only: setting.index_display_mode == IndexDisplayMode::LovedOnly,
      scale: setting.scale.get(),
    }
  }
}

#[derive(Serialize)]
struct UserData {
  username: String,
  follows: Vec<FollowData>,
}

#[derive(Serialize)]
struct FollowData {
  liked: bool,
  has_update: bool,
  max_read_episode: u32,
  manga: MangaData,
}

#[derive(Serialize)]
struct MangaData {
  title: String,
  dmk_id: String,
  cover_url: String,
  saemanga_url: String,
  last_episode: i32,
  last_episode_is_book: bool,
  ended: bool,
}

#[derive(Serialize)]
struct GenreData {
  id: &'static str,
  name: &'static str,
}

impl From<&&Genre> for GenreData {
  fn from(genre: &&Genre) -> Self {
    Self { id: genre.id, name: genre.name }
  }
}

#[get("/index")]
pub fn index(user: Option<&User>, conn: Database, setting: UserSetting) -> Template {

  // Create temporary data
  let data = TemplateData {
    latests: Manga::get_latest_10(&conn).unwrap().into_iter().map(|m| {
      let data = m.data();
      MangaData {
        title: data.title().clone(),
        dmk_id: data.dmk_id().clone(),
        cover_url: data.dmk_cover_url(),
        saemanga_url: data.saemanga_url(),
        last_episode: data.latest_episode().episode(),
        last_episode_is_book: data.latest_episode().is_book(),
        ended: data.ended()
      }
    }).collect(),
    user: user.map(|user| UserData {
      username: user.username().clone(),
      follows: vec![],
    }),
    genres: Genre::all_genres().iter().map(GenreData::from).collect(),
    setting: SettingData::from(setting)
  };

  // Render the data
  Template::render("index", &data)
}