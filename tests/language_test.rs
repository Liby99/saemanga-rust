use encoding::{EncoderTrap, DecoderTrap};
use encoding::all::BIG5_2003;
use encoding::types::Encoding;

#[test]
fn test_big5_encode() {
  let string = String::from("鬱悶飯");
  println!("{:X?}", BIG5_2003.encode(&string, EncoderTrap::Ignore));
}

#[test]
fn test_big5_encode_2() {
  let string = String::from("湯搖莊的幽奈同學");
  println!("{:X?}", BIG5_2003.encode(&string, EncoderTrap::Ignore));
}

#[test]
fn test_big5_decode() {
  let arr : [u8; 6] = [0xC6, 0x7B, 0xB4, 0x65, 0xB6, 0xBA];
  println!("{:X?}", BIG5_2003.decode(&arr, DecoderTrap::Ignore));
}

// #[test]
// fn test_big5_decode_2() {
//   let arr : [u8; 6] = [0xC6, 0x7B, 0xB4, 0x65, 0xB6, 0xBA];
//   println!("{:X?}", BIG5_2003.decode(&arr, DecoderTrap::Ignore));
// }