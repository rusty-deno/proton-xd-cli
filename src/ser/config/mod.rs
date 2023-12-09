#![allow(dead_code)]
mod unstable;
mod permission;
mod compiler_options;
mod config;

use unstable::*;
use permission::*;
use compiler_options::*;
pub(crate) use config::*;



use std::collections::LinkedList;
use super::Writer;

use serde::{
  Serialize,
  Deserialize
};



#[derive(Deserialize,Serialize,Debug)]
pub(crate) enum Value {
  #[serde(rename="*")]
  All,
  Bool(bool),
  Vec(Box<[Box<str>]>)
}
pub(crate) type Val=Option<Value>;


pub(crate) trait ToArgs {
  fn to_flags<'a>(self)-> LinkedList<&'a str>;
}






#[cfg(test)]
mod tests {
  use crate::ser::config::Config;

  #[test]
  fn xd() {
    let config: Config=serde_json::from_str(&std::fs::read_to_string("./proton-config.json").unwrap()).unwrap();
    println!("{config:?}")
  }
}
