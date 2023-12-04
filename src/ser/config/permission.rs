use serde::Deserialize;

use super::{
  Value::Vec,
  Val
};


pub(crate) const FFI_DIR: &str="https://github.com/kakashi-69-xd/proton-xd/bindings/bin";


#[derive(Deserialize,Debug)]
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

