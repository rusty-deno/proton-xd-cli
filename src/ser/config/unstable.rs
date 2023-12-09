
use serde::{
  Deserialize,
  Serialize
};

use super::ToArgs;
use std::collections::LinkedList;


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
  fn to_flags<'a>(self)-> LinkedList<&'a str> {
    use Unstable::*;
    match self {
      Bool(false)=> LinkedList::new(),
      All|Bool(true)=> LinkedList::from_iter(["--unstable"]),
      Explicit(options)=> {
        let mut list=LinkedList::<&'a str>::new();

        for option in options.iter() {
          list.push_back(option.to_flag());
        }
        
        list
      },
    }
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

impl UnstableOption {
  fn to_flag<'a>(&self)-> &'a str {
    use UnstableOption::*;
    match self {
      BareNodeBuiltins=> "--unstable-bare-node-builtins",
      Byonm=> "--unstable-byonm",
      Workspces=> "--unstable-workspces",
      Broadcast=> "--unstable-broadcast",
      Ffi=> "--unstable-ffi",
      Fs=> "--unstable-fs",
      Kv=> "--unstable-kv",
      Net=> "--unstable-net",
      Http=> "--unstable-http",
      WorkerOptions=> "--unstable-worker-options",
      Cron=> "--unstable-cron",
    }
  }
}


