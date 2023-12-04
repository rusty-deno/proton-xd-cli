
use serde::{
  Deserialize,
  Serialize
};


#[derive(Deserialize,Debug)]
#[serde(rename_all="kebab-case")]
pub(crate) enum Unstable {
  #[serde(rename="*")]
  All,
  Bool(bool),
  Explicit(Box<[UnstableOption]>)
}

impl Unstable {
  pub fn new()-> Self {
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


