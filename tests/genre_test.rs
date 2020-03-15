use saemanga::app::genre::*;

#[test]
fn test_genre_for_id() {
  assert_eq!(Genre::for_id("adv"), Some(&ADV));
  assert_eq!(Genre::for_id("shonen"), Some(&SHONEN));
  assert_eq!(Genre::for_id("magic"), Some(&MAGIC));
  assert_eq!(Genre::for_id("sport"), Some(&SPORT));
  assert_eq!(Genre::for_id(""), None);
}
