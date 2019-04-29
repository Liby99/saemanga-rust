use saemanga::app::manga::*;

#[test]
fn test_parse_dmk_image_url() {
  let url = String::from("https://web3.cartoonmad.com/w93ug014/9543/001/001.jpg");
  assert_eq!(
    MangaDmkIdBase::from_dmk_image_url(&url),
    Some(MangaDmkIdBase::V05 {
      dmk_id_web: String::from("web3"),
      dmk_id_gen: String::from("w93ug014")
    })
  );
}

#[test]
fn test_parse_dmk_image_url_2() {
  let url = String::from("https://www.cartoonmad.com/home75466/4897/008/002.jpg");
  assert_eq!(
    MangaDmkIdBase::from_dmk_image_url(&url),
    Some(MangaDmkIdBase::V09 { dmk_id_home: String::from("home75466") })
  );
}