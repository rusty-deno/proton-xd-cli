use std::path::PathBuf;

use serde::{
  Deserialize,
  Serialize
};

use super::{
  Val,
  ToArgs
};



fn _true()-> bool {
  true
}

#[derive(Deserialize,Serialize,Debug)]
#[serde(rename_all="kebab-case")]
pub(crate) struct CompilerOptions {
  pub(crate) no_check: Val,
  pub(crate) import_map: Option<PathBuf>,
  pub(crate) no_remote: Option<bool>,
  pub(crate) no_npm: Option<bool>,
  pub(crate) node_modules_dir: Option<Box<[PathBuf]>>,
  pub(crate) vendor: Option<bool>,
  pub(crate) config: Option<PathBuf>,
  pub(crate) reload: Option<Box<[PathBuf]>>,
  pub(crate) lock: Option<PathBuf>,
  pub(crate) lock_write: Option<bool>,
  pub(crate) no_lock: Option<bool>,
  pub(crate) cert: Option<PathBuf>,
  pub(crate) quiet: Option<bool>,
  pub(crate) unsafely_ignore_certificate_errors: Option<Box<[PathBuf]>>,
  #[serde(default="_true")]
  pub(crate) no_prompt: bool,
  pub(crate) catch_only: Option<bool>,
  pub(crate) location: Option<PathBuf>,
  pub(crate) v8_flags: Option<Box<[Box<str>]>>,
  pub(crate) seed: Option<u128>,
  pub(crate) check: Option<Box<[PathBuf]>>,
  pub(crate) include: Option<PathBuf>,
  pub(crate) output: Option<PathBuf>,
  pub(crate) target: Option<Box<str>>,
  #[serde(default="_true")]
  pub(crate) no_terminal: bool,
  pub(crate) env: Option<PathBuf>,
}

impl CompilerOptions {
  pub fn new()-> Self {
    Default::default()
  }
}

impl Default for CompilerOptions {
  fn default()-> Self {
    Self {
      no_prompt: true,
      no_terminal: true,
      catch_only: Some(true),
      // none
      no_check: None,
      import_map: None,
      no_remote: None,
      no_npm: None,
      node_modules_dir: None,
      vendor: None,
      config: None,
      reload: None,
      lock: None,
      lock_write: None,
      no_lock: None,
      cert: None,
      quiet: None,
      unsafely_ignore_certificate_errors: None,
      location: None,
      v8_flags: None,
      seed: None,
      check: None,
      include: None,
      output: None,
      target: None,
      env: None
    }
  }
}


impl ToArgs for CompilerOptions {
  fn to_flags<'a>(self)-> std::collections::linked_list::IntoIter<&'a str> {
    unimplemented!()
  }
}

