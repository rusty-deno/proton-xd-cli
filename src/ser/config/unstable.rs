
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
