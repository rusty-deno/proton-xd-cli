#![allow(dead_code)]
mod unstable;
mod permission;
mod compiler_options;

use unstable::*;
use permission::*;
use compiler_options::*;


use serde::Deserialize;





#[derive(Deserialize,Debug)]
pub(crate) enum Value {
  #[serde(rename="*")]
  All,
  Bool(bool),
  Vec(Box<[Box<str>]>)
}
pub(crate) type Val=Option<Value>;


#[derive(Deserialize,Debug)]
#[serde(rename_all="kebab-case")]
pub(crate) struct Config {
  compiler_options: CompilerOptions,
  permissions: Permissions,
  unstable: Unstable
}













#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;


  #[test]
  fn xd() {
    let config: Config=serde_json::from_str(&fs::read_to_string("./proton-config.json").unwrap()).unwrap();




    println!("{config:?}")
  }

}
