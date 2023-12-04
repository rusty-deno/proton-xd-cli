#![allow(dead_code)]
mod unstable;
mod permission;
mod compiler_options;


use unstable::*;
use permission::*;
use compiler_options::*;


use tokio::*;
use std::env;
use serde::Deserialize;
use std::path::PathBuf;




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
  name: Box<str>,
  version: Box<str>,
  compiler_options: CompilerOptions,
  permissions: Permissions,
  unstable: Unstable
}

impl Config {
  pub fn new()-> Config {
    Config {
      compiler_options: CompilerOptions::new(),
      name: "".into(),
      version: "".into(),
      permissions: Permissions::new(),
      unstable: Unstable::new()
    }
  }

  pub async fn fetch_config()-> io::Result<(Config,PathBuf)> {
    loop {
      let res=fs::read_to_string("./proton-config.json").await;

      if let Ok(res)=res {
        return Ok((serde_json::from_str(&res).unwrap(),env::current_dir()?));
      }
      
      match res.unwrap_err().kind() {
        io::ErrorKind::NotFound=> env::set_current_dir("..")?,
        _=> panic!("No `proton-xd-config.json` file found!")
      }
    }
  }
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
