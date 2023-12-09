
use tokio::*;
use super::*;
use crate::CONFIG_FILE_NAME;

use std::{
  env,
  path::Path,
  collections::LinkedList
};

use serde::{
  Serialize,
  Deserialize
};




#[derive(Deserialize,Serialize,Debug)]
#[serde(rename_all="kebab-case")]
pub(crate) struct Config {
  pub(crate) name: Box<str>,
  pub(crate) version: Box<str>,
  pub(crate) compiler_options: CompilerOptions,
  pub(crate) permissions: Permissions,
  pub(crate) unstable: Unstable
}

impl Config {
  pub fn new(name: &str)-> Config {
    Config {
      name: name.into(),
      ..Default::default()
    }
  }

  /// finds the config file and switches to that directory
  pub async fn find_config_file()-> io::Result<Config> {
    loop {
      let res=fs::read_to_string(CONFIG_FILE_NAME).await;

      if let Ok(res)=res {
        return Ok(serde_json::from_str(&res).unwrap());
      }
      
      match res.unwrap_err().kind() {
        io::ErrorKind::NotFound=> env::set_current_dir(".."),
        kind=> Err(io::Error::new(kind,format!("No `{CONFIG_FILE_NAME}` file found!")))
      }?
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


impl ToArgs for Config {
  fn to_flags<'a>(self)-> LinkedList<&'a str> {
    let mut flags=self.compiler_options.to_flags();
    flags.append(&mut self.permissions.to_flags());
    flags.append(&mut self.unstable.to_flags());
    
    flags
  }
}





