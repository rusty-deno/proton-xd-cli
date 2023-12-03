use serde::Deserialize;
use super::Val;



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

