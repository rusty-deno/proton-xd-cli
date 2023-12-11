use std::collections::{LinkedList, HashMap};

use super::{
  Str,
  Parse,
  Unstable
};




pub(crate) trait ToArgs {
  fn to_flags(self)-> LinkedList<Box<str>>;
}


impl<V: Parse> ToArgs for HashMap<Str,V> {
  fn to_flags(self)-> LinkedList<Box<str>> {
    self.iter().map(|(k,v)| v.parse(&format!("--{k}"))).collect()
  }
}


impl ToArgs for Unstable {
  fn to_flags(self)-> LinkedList<Box<str>> {
    self.iter().map(|feature| format!("--unstable-{feature}").into()).collect()
  }
}

