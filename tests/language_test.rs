use encoding::{EncoderTrap, DecoderTrap};
use encoding::all::BIG5_2003;
use encoding::types::Encoding;

#[test]
fn test_big5_encode() {
  let string = String::from("鬱悶飯");
  assert_eq!(BIG5_2003.encode(&string, EncoderTrap::Ignore), Ok(vec![0xC6, 0x7B, 0xB4, 0x65, 0xB6, 0xBA]));
}

#[test]
fn test_big5_encode_2() {
  let string = String::from("湯搖莊的幽奈同學");
  println!("{:X?}", BIG5_2003.encode(&string, EncoderTrap::Ignore));
}

#[test]
fn test_big5_decode() {
  let arr : [u8; 6] = [0xC6, 0x7B, 0xB4, 0x65, 0xB6, 0xBA];
  assert_eq!(BIG5_2003.decode(&arr, DecoderTrap::Ignore), Ok(String::from("鬱悶飯")));
}

#[test]
fn test_big5_decode_2() {
  let arr : [u8; 16] = [0xB4, 0xF6, 0xB7, 0x6E, 0xB2, 0xF8, 0xAA, 0xBA, 0xAB, 0xD5, 0xA9, 0x60, 0xA6, 0x50, 0xBE, 0xC7];
  assert_eq!(BIG5_2003.decode(&arr, DecoderTrap::Ignore), Ok(String::from("湯搖莊的幽奈同學")));
}