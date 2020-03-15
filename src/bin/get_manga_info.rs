use std::env;

use saemanga::app::dmk::fetch_manga_data;
use saemanga::app::manga::Manga;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() == 1 {
    println!("Expecting argument: dmk_id");
  } else {
    let dmk_id = &args[1];
    match fetch_manga_data(dmk_id) {
      Ok(manga_data) => match Manga::new(&manga_data) {
        Ok(manga) => match serde_json::to_string(&manga) {
          Ok(json_str) => println!("{}", json_str),
          Err(err) => println!("{:?}", err),
        },
        Err(err) => println!("{:?}", err),
      },
      Err(err) => println!("{:?}", err),
    }
  }
}
