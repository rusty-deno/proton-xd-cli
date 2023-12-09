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
      Permissions::Permissions {..}=> {
        let mut _flags=LinkedList::new();








        _flags
      }
    }
  }
}



fn parse<'a>(val: Val,perm: &str)-> Box<str> {
  match val {
    None=> "".into(),
    Some(v)=> {
      match v {
        All|Bool(true)=> format!("--allow-{perm}").into_boxed_str(),
        Bool(_)=> todo!(),
        Vec(_)=> todo!(),
      }
    },
  }
}
