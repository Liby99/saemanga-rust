use std::Option;

struct MangaBase {

  // The id which looks like `6371`, `1286`
  dmk_id: u32,

  // Additional dmk 
  dmk_id: MangaUrlComponent,

  // Title of the manga
  title: String,

  // 
  description: String,

  // 
  author: String,

  // 
}

pub trait MangaToUrl {
  pub fn get_saemanga_base_url();
  pub fn get_saemanga_url(episode: u32);
  pub fn get_dmk_base_url();
  pub fn get_dmk_episode_url(episode: u32);
  pub fn get_dmk_image_url(episode: u32, page: u32);
}

// pub enum MangaUrlComponent {
//   One { dmk_id_web: String, dmk_id_gen: String },
//   Two { dmk_id_gen: String },
//   Three { dmk_id_gen: String },
//   Four { dmk_id_home: String },
//   Five { dmk_id_home: String },
//   Six { dmk_id_web: String, dmk_id_home: String },
// }