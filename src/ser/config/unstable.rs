
use serde::{
  Deserialize,
  Serialize
};

use super::*;
use std::collections::LinkedList;


#[derive(Deserialize,Serialize,Debug)]
#[serde(rename_all="kebab-case")]
pub(crate) struct Unstable {
  pub(crate) bare_node_builtins: Option<bool>,
  pub(crate) byonm: Option<bool>,
  pub(crate) workspces: Option<bool>,
  pub(crate) broadcast: Option<bool>,
  pub(crate) ffi: Option<bool>,
  pub(crate) fs: Option<bool>,
  pub(crate) kv: Option<bool>,
  pub(crate) net: Option<bool>,
  pub(crate) http: Option<bool>,
  pub(crate) worker_options: Option<bool>,
  pub(crate) cron: Option<bool>
}

impl Default for Unstable {
  fn default()-> Self {
    Self {
      bare_node_builtins: None,
      byonm: None,
      workspces: None,
      broadcast: None,
      ffi: Some(true),
      fs: None,
      kv: None,
      net: None,
      http: None,
      worker_options: None,
      cron: None
    }
  }
}

impl ToArgs for Unstable {
  fn to_flags(&self)-> LinkedList<Option<Box<str>>> {
    LinkedList::from_iter([
      self.bare_node_builtins.parse("--unstable-bare-node-builtins"),
      self.byonm.parse("--unstable-byonm"),
      self.workspces.parse("--unstable-workspces"),
      self.broadcast.parse("--unstable-broadcast"),
      self.ffi.parse("--unstable-ffi"),
      self.fs.parse("--unstable-fs"),
      self.kv.parse("--unstable-kv"),
      self.net.parse("--unstable-net"),
      self.http.parse("--unstable-http"),
      self.worker_options.parse("--unstable-worker-options"),
      self.cron.parse("--unstable-cron")
    ])
  }
}



