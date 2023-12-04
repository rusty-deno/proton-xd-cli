use super::Val;
use serde::Deserialize;
use std::path::PathBuf;


fn _true()-> bool {
  true
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
  target: Option<Box<str>>,
  #[serde(default="_true")]
  no_terminal: bool,
  env: Option<PathBuf>,
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


