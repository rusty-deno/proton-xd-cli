use std::collections::{LinkedList, HashMap};

use super::{
  Str,
  Parse,
  Array, Unstable
};




pub(crate) trait ToArgs {
  fn to_flags(&self)-> LinkedList<Option<Box<str>>>;

  fn parse_args(&self)-> LinkedList<Box<str>> {
    let mut list=LinkedList::new();

    for arg in self.to_flags() {
      match arg {
        Some(arg)=> list.push_back(arg),
        _=> continue
      }
    }
    
    list
  }
}


impl ToArgs for HashMap<Str,Array<Str>> {
  fn to_flags(&self)-> LinkedList<Option<Box<str>>> {
    self.iter().map(|(k,v)| v.parse(&format!("--{k}"))).collect()
  }
}


impl ToArgs for Unstable {
  fn to_flags(&self)-> LinkedList<Option<Box<str>>> {
    self.iter().map(|(feature,val)|
      //example format!("--unstable-{ffi}")
      val.parse(&format!("--unstable-{feature}"))
    ).collect()
  }
}

