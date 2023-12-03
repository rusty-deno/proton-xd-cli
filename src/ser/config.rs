#![allow(dead_code)]

use std::path::PathBuf;
use serde::{
  Deserialize,
  Serialize
};


#[derive(Deserialize,Debug)]
pub(crate) enum Value {
  #[serde(rename="*")]
  All,
  Bool(bool),
  Vec(Box<[Box<str>]>)
}
type Val=Option<Value>;




#[derive(Deserialize,Debug)]
#[serde(rename_all="kebab-case")]
pub(crate) struct Config {
  compiler_options: CompilerOptions,
  permissions: Permissions,
  unstable: Unstable
}


#[derive(Deserialize,Debug)]
#[serde(rename_all="kebab-case")]
pub(crate) enum Unstable {
  #[serde(rename="*")]
  All,
  Bool(bool),
  Others(Box<[UnstableOption]>)
}

#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all="kebab-case")]
pub(crate) enum UnstableOption {
  BareNodeBuiltins,
  Byonm,
  Workspces,
  Broadcast,
  Ffi,
  Fs,
  Kv,
  Net,
  Http,
  WorkerOptions,
  Cron
}


#[derive(Deserialize,Debug)]
#[serde(rename_all="kebab-case")]
pub(crate) enum Permissions {
  #[serde(rename="*")]
  All,
  Permissions {
    //allow
    allow_read: Val,
    allow_write: Val,
    allow_net: Val,
    unsafely_ignore_certificate_errors: Val,
    allow_env: Val,
    allow_sys: Val,
    allow_ffi: Val,
    allow_hrtm: Val,
    //deny
    deny_read: Val,
    deny_write: Val,
    deny_net: Val,
    deny_env: Val,
    deny_sys: Val,
    deny_ffi: Val,
    deny_hrtm: Val
  }
}



#[derive(Deserialize,Debug)]
#[serde(rename_all="kebab-case")]
pub(crate) struct CompilerOptions {
  no_check: Val,
  import_map: Option<PathBuf>,
  no_remote: Option<bool>,
  no_npm: Option<bool>,
  node_modules_dir: Option<Box<[PathBuf]>>,
  vendor: Option<bool>,
  config: Option<PathBuf>,
  reload: Option<Box<[PathBuf]>>,
  lock: Option<PathBuf>,
  lock_write: Option<bool>,
  no_lock: Option<bool>,
  cert: Option<PathBuf>,
  quiet: Option<bool>,
  unsafely_ignore_certificate_errors: Option<Box<[PathBuf]>>,
  #[serde(default="_true")]
  no_prompt: bool,
  catch_only: Option<bool>,
  location: Option<PathBuf>,
  v8_flags: Option<Box<[Box<str>]>>,
  seed: Option<u128>,
  check: Option<Box<[PathBuf]>>,
  include: Option<PathBuf>,
  output: Option<PathBuf>,
  target: Option<Box<str>>,//type refactor
  #[serde(default="_true")]
  no_terminal: bool,
  env: Option<PathBuf>,
}

fn _true()-> bool {
  true
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
