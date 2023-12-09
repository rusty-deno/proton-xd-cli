use crate::ser::FFI_DIR;
use std::collections::LinkedList;

use super::{
  Value::*,
  Val,
  ToArgs
};

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
      allow_ffi: Some(Vec([FFI_DIR.into()].into())),// path of dylib
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
          parse(allow_read,"--allow-read"),
          parse(allow_write,"--allow-write"),
          parse(allow_net,"--allow-net"),
          parse(allow_env,"--allow-env"),
          parse(allow_sys,"--allow-sys"),
          parse(allow_ffi,"--allow-ffi"),
          parse(allow_hrtm,"--allow-hrtm"),
          parse(unsafely_ignore_certificate_errors,"--unsafely-ignore-certificate-errors"),
          parse(deny_read,"--deny-read"),
          parse(deny_write,"--deny-write"),
          parse(deny_net,"--deny-net"),
          parse(deny_env,"--deny-env"),
          parse(deny_sys,"--deny-sys"),
          parse(deny_ffi,"--deny-ffi"),
          parse(deny_hrtm,"--deny-hrtm"),
        ])
      }
    }
  }
}



fn parse<'a>(val: Val,perm: &str)-> Box<str> {
  match val {
    None=> "".into(),
    Some(v)=> {
      match v {
        // allow_read: ["/home","/dev"] turns into --allow-read="/home,/dev"
        Vec(list)=> format!("{perm}=\"{}\"",list.join(",")).into_boxed_str(),
        All|Bool(true)=> format!("{perm}").into_boxed_str(),
        _=> "".into(),
      }
    },
  }
}
