#![feature(custom_attribute)]

#[collection("user")]
pub struct User {
  username: String,
  password: String,
}