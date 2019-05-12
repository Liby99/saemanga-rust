#![feature(custom_attribute)]

#[macro_use] extern crate collection;

#[collection("user")]
pub struct User {
  username: String,
  password: String,
}