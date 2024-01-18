
use tokio::*;
use super::*;
use super::super::consts::*;

use std::{
  env,
  path::Path,
  collections::LinkedList,
  io::{
    Error,
    ErrorKind::NotFound
  }
};

use serde::{
  Serialize,
  Deserialize
};




#[derive(Deserialize,Serialize,Debug)]
#[serde(rename_all="kebab-case")]
pub(crate) struct Config {
  pub(crate) name: Str,
  pub(crate) main: Option<Str>,
  pub(crate) language: Option<Language>,
  pub(crate) compiler_options: CompilerOptions,
  pub(crate) permissions: Permissions,
  pub(crate) unstable: Unstable
}


#[derive(Deserialize,Serialize,Debug,Default)]
#[serde(rename_all="lowercase")]
pub(crate) enum Language {
  #[default]
  TypeScript,Ts,
  JavaScript,Js
}

impl Language {
  pub(crate) fn extension<'a>(self)-> &'a str {
    use Language::*;
    match self {
      TypeScript|Ts=> "ts",
      JavaScript|Js=> "js",
    }
  }
}



impl Config {
  pub(crate) fn new(name: &str)-> Config {
    Config {
      name: name.into(),
      ..Default::default()
    }
  }

  /// finds the config file and switches to that directory
  pub(crate) async fn find_config_file()-> io::Result<Config> {
    let not_found: &str=&format!("No `{CONFIG_FILE_NAME}` file found!");

    while let Some(_)=env::current_dir()?.parent() {
      let res=fs::read_to_string(CONFIG_FILE_NAME).await;

      if let Ok(res)=res {
        return Ok(serde_json::from_str(&res)?);
      }
      
      match res.unwrap_err().kind() {
        NotFound=> env::set_current_dir(".."),
        kind=> Err(Error::new(kind,not_found))
      }?
    }

    Err(Error::new(NotFound,not_found))
  }

  pub(crate) async fn save<P: AsRef<Path>>(self,path: P)-> io::Result<()> {
    let mut w=Writer::new();
    let mut serializer=serde_json::Serializer::pretty(&mut w);
    self.serialize(&mut serializer)?;

    fs::write(path,w.to_vec()).await
  }
}


impl Default for Config {
  fn default()-> Self {
    Self {
      name: "my-app".into(),
      main: Some(MAIN.into()),
      language: Some(Default::default()),
      compiler_options: Default::default(),
      permissions: Permissions::default(),
      unstable: Unstable::default()
    }
  }
}


impl ToArgs for Config {
  fn to_flags(&self)-> LinkedList<Option<Str>> {
    let mut flags=self.compiler_options.to_flags();
    flags.append(&mut self.permissions.to_flags());
    flags.append(&mut self.unstable.to_flags());
    
    flags
  }
}





