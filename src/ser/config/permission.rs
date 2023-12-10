
use super::*;
use crate::ser::FFI_DIR;
use std::collections::LinkedList;


use serde::{
  Deserialize,
  Serialize
};




#[derive(Deserialize,Serialize,Debug)]
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


impl Permissions {
  pub fn new()-> Self {
    Self::default()
  }
}

impl Default for Permissions {
  fn default()-> Self {
    Self::Permissions {
      //allow
      allow_read: None,
      allow_write: None,
      allow_net: None,
      unsafely_ignore_certificate_errors: None,
      allow_env: None,
      allow_sys: None,
      allow_ffi: Some(Value::Vec([FFI_DIR.into()].into())),// path of dylib
      allow_hrtm: None,
      //deny
      deny_read: None,
      deny_write: None,
      deny_net: None,
      deny_env: None,
      deny_sys: None,
      deny_ffi: None,
      deny_hrtm: None,
    }
  }
}


impl ToArgs for Permissions {
  fn to_flags(self)-> LinkedList<Box<str>> {

    match self {
      Permissions::All=> LinkedList::from_iter(["-A".into()]),
      Permissions::Permissions {
        allow_read,
        allow_write,
        allow_net,
        unsafely_ignore_certificate_errors,
        allow_env,
        allow_sys,
        allow_ffi,
        allow_hrtm,
        deny_read,
        deny_write,
        deny_net,
        deny_env,
        deny_sys,
        deny_ffi,
        deny_hrtm
      }=> {
        LinkedList::from_iter([
          allow_read.parse("--allow-read"),
          allow_write.parse("--allow-write"),
          allow_net.parse("--allow-net"),
          allow_env.parse("--allow-env"),
          allow_sys.parse("--allow-sys"),
          allow_ffi.parse("--allow-ffi"),
          allow_hrtm.parse("--allow-hrtm"),
          unsafely_ignore_certificate_errors.parse("--unsafely-ignore-certificate-errors"),
          deny_read.parse("--deny-read"),
          deny_write.parse("--deny-write"),
          deny_net.parse("--deny-net"),
          deny_env.parse("--deny-env"),
          deny_sys.parse("--deny-sys"),
          deny_ffi.parse("--deny-ffi"),
          deny_hrtm.parse("--deny-hrtm"),
        ])
      }
    }
  }
}




