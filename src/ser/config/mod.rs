#![allow(dead_code)]
mod unstable;
mod permission;
mod compiler_options;

use unstable::*;
use permission::*;
use compiler_options::*;

use tokio::*;
use super::Writer;
use std::path::PathBuf;
use crate::CONFIG_FILE_NAME;

use std::{
  env,
  path::Path
};

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


#[derive(Deserialize,Serialize,Debug)]
#[serde(rename_all="kebab-case")]
pub(crate) struct Config {
  name: Box<str>,
  version: Box<str>,
  compiler_options: CompilerOptions,
  permissions: Permissions,
  unstable: Unstable
}

impl Config {
  pub fn new(name: &str)-> Config {
    Config {
      name: name.into(),
      ..Default::default()
    }
  }

  pub async fn fetch_config()-> io::Result<(Config,PathBuf)> {
    loop {
      let res=fs::read_to_string(CONFIG_FILE_NAME).await;

      if let Ok(res)=res {
        return Ok((serde_json::from_str(&res).unwrap(),env::current_dir()?));
      }
      
      match res.unwrap_err().kind() {
        io::ErrorKind::NotFound=> env::set_current_dir("..")?,
        _=> panic!("No `{CONFIG_FILE_NAME}` file found!")
      }
    }
  }

  pub async fn save<P: AsRef<Path>>(self,path: P)-> io::Result<()> {
    let mut w=Writer::new();
    let mut serializer=serde_json::Serializer::new(&mut w);
    self.serialize(&mut serializer)?;

    fs::write(path,w.to_vec()).await
  }
}


impl Default for Config {
  fn default()-> Self {
    Self {
      name: "my-app".into(),
      version: "1.0.0".into(),
      compiler_options: Default::default(),
      permissions: Default::default(),
      unstable: Default::default()
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
