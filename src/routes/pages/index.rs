use rocket_contrib::templates::Template;
use rocket::http::Cookies;

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
      scale: setting.scale,
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
  last_episode: u32,
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
pub fn index(user: Option<&User>, cookies: Cookies) -> Template {

  // Create temporary data
  let data = TemplateData {
    latests: vec![
      MangaData {
        title: String::from("可憐可愛元氣君"),
        dmk_id: String::from("8193"),
        cover_url: String::from("http://cartoonmad.com/cartoonimgs/coimg/8193.jpg"),
        saemanga_url: String::from("http://saemanga.com/manga/8193"),
        last_episode: 7,
        last_episode_is_book: false,
        ended: false,
      },
      MangaData {
        title: String::from("家庭教師"),
        dmk_id: String::from("1254"),
        cover_url: String::from("http://cartoonmad.com/cartoonimgs/coimg/1254.jpg"),
        saemanga_url: String::from("http://saemanga.com/manga/1254"),
        last_episode: 409,
        last_episode_is_book: false,
        ended: true,
      },
      MangaData {
        title: String::from("聖癖✟櫻之丘"),
        dmk_id: String::from("5901"),
        cover_url: String::from("http://cartoonmad.com/cartoonimgs/coimg/5901.jpg"),
        saemanga_url: String::from("http://saemanga.com/manga/5901"),
        last_episode: 26,
        last_episode_is_book: false,
        ended: false,
      },
      MangaData {
        title: String::from("田中君總是如此慵懶"),
        dmk_id: String::from("4159"),
        cover_url: String::from("http://cartoonmad.com/cartoonimgs/coimg/4159.jpg"),
        saemanga_url: String::from("http://saemanga.com/manga/4159"),
        last_episode: 26,
        last_episode_is_book: false,
        ended: false,
      },
    ],
    user: user.map(|user| UserData {
      username: user.username().clone(),
      follows: vec![],
    }),
    genres: Genre::all().iter().map(GenreData::from).collect(),
    setting: SettingData::from(UserSetting::from_cookies(&cookies))
  };

  // Render the data
  Template::render("index", &data)
}