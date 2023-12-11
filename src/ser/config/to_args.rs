use std::collections::{LinkedList, HashMap};

use super::{
  Str,
  Parse,
  Array, Unstable
};




pub(crate) trait ToArgs {
  fn to_flags(&self)-> LinkedList<Box<str>>;

  fn parse_args(&self)-> LinkedList<Box<str>> {
    // self.to_flags().iter().filter(|val| )
    self.to_flags()
  }
}


impl ToArgs for HashMap<Str,Array<Str>> {
  fn to_flags(&self)-> LinkedList<Box<str>> {
    self.iter().map(|(k,v)| v.parse(&format!("--{k}"))).collect()
  }
}


impl ToArgs for Unstable {
  fn to_flags(&self)-> LinkedList<Box<str>> {
    self.iter().map(|(feature,val)|
      //example format!("--unstable-{ffi}")
      val.parse(&format!("--unstable-{feature}"))
    ).collect()
  }
}

