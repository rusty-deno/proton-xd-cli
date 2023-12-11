
use super::*;
use crate::ser::FFI_DIR;
use std::collections::LinkedList;


use serde::{
  Deserialize,
  Serialize
};

#[derive(Deserialize,Serialize,Debug)]
#[serde(untagged)]
pub(crate) enum Permission {
  Bool(bool),
  Permissions(Array<Str>)// Array<T>=Box<[T]>, Str=Box<str>
}

pub(crate) type Perm=Option<Permission>;


#[derive(Deserialize,Serialize,Debug)]
#[serde(rename_all="kebab-case",untagged)]
pub(crate) enum Permissions {
  Bool(bool),
  Permissions {
    //allow
    allow_read: Perm,
    allow_write: Perm,
    allow_net: Perm,
    unsafely_ignore_certificate_errors: Perm,
    allow_env: Perm,
    allow_sys: Perm,
    allow_ffi: Perm,
    allow_hrtm: Perm,
    //deny
    deny_read: Perm,
    deny_write: Perm,
    deny_net: Perm,
    deny_env: Perm,
    deny_sys: Perm,
    deny_ffi: Perm,
    deny_hrtm: Perm
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
      allow_ffi: Some(Permission::Permissions([FFI_DIR.into()].into())),// path of dylib
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
  fn to_flags(&self)-> LinkedList<Option<Box<str>>> {

    match self {
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
      Permissions::Bool(true)=> LinkedList::from_iter([Some("-A".into())]),
      _=> LinkedList::new()
    }
  }
}



impl Parse for Permission {
  fn parse(&self,option: &str)-> Option<Box<str>> {
    match self {
      //                                   # Example format!("{--allow-read}=\"{/home,/dev}\""); --allow-read="/home,/dev"
      Permission::Permissions(perms)=> Some(format!("{option}=\"{}\"",perms.join(",")).into_boxed_str()),
      Permission::Bool(true)=> Some(option.into()),
      _=> None
    }
  }
}

