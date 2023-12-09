
use serde::{
  Deserialize,
  Serialize
};

use super::ToArgs;
use std::collections::linked_list::IntoIter;


#[derive(Deserialize,Serialize,Debug)]
#[serde(rename_all="kebab-case")]
pub(crate) enum Unstable {
  #[serde(rename="*")]
  All,
  Bool(bool),
  Explicit(Box<[UnstableOption]>)
}

impl Unstable {
  pub fn new()-> Self {
    Self::default()
  }
}

impl Default for Unstable {
  fn default()-> Self {
    Self::Explicit([UnstableOption::Ffi].into())
  }
}

impl ToArgs for Unstable {
  fn to_flags<'a>(self)-> IntoIter<&'a str> {
    unimplemented!()
  }
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


impl ToArgs for UnstableOption {
  fn to_flags<'a>(self)-> IntoIter<&'a str> {
    unimplemented!()
  }
}

