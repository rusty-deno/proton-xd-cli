#![allow(dead_code)]
mod unstable;
mod permission;
mod compiler_options;
mod config;

use unstable::*;
use permission::*;
use compiler_options::*;
pub(crate) use config::*;


use super::Writer;

use serde::{
  Serialize,
  Deserialize
};

use std::{
  collections::LinkedList,
  path::Path
};


#[derive(Deserialize,Serialize,Debug)]
pub(crate) enum Value {
  #[serde(rename="*")]
  All,
  Bool(bool),
  Vec(Box<[Box<str>]>)
}
pub(crate) type Val=Option<Value>;

pub(crate) trait ToArgs {
  fn to_flags(self)-> LinkedList<Box<str>>;
}

pub(in crate::ser::config) trait Parse {
  fn parse(self,option: &str)-> Box<str>;
}

impl Parse for Val {
  fn parse(self,option: &str)-> Box<str> {
    use Value::*;
    match self {
      None=> "".into(),
      Some(v)=> {
        match v {
          // allow_read: ["/home","/dev"] turns into --allow-read="/home,/dev"
          Vec(list)=> format!("{option}=\"{}\"",list.join(",")).into_boxed_str(),
          All|Bool(true)=> format!("{option}").into_boxed_str(),
          _=> "".into(),
        }
      },
    }
  }
}


impl<S: Parse> Parse for Option<S> {
  fn parse(self,option: &str)-> Box<str> {
    match self {
      Some(val)=> val.parse(option),
      _=> "".into()
    }
  }
}





impl Parse for bool {
  fn parse(self,option: &str)-> Box<str> {
    match self {
      true=> option,
      _=> ""
    }.into()
  }
}

impl Parse for u128 {
  fn parse(self,option: &str)-> Box<str> {
    format!("{option} {self}").into_boxed_str()
  }
}



impl Parse for Box<Path> {
  fn parse(self,option: &str)-> Box<str> {
    format!("{option}={}",self.display()).into_boxed_str()
  }
}

impl Parse for Box<[Box<str>]> {
  fn parse(self,option: &str)-> Box<str> {
    format!("{option}=\"{}\"",self.join(",")).into_boxed_str()
  }
}

impl Parse for Box<str> {
  fn parse(self,option: &str)-> Box<str> {
    format!("{option} {}",&self).into_boxed_str()
  }
}



#[cfg(test)]
mod tests {

  #[test]
  fn xd() {











  }
}
