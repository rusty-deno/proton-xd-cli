mod compiler_options;
mod config;
mod to_args;

use compiler_options::*;
pub(crate) use config::*;
pub(crate) use to_args::*;


use super::Writer;

use serde::{
  Serialize,
  Deserialize
};

use std::path::Path;


#[derive(Deserialize,Serialize,Debug)]
pub(crate) enum Value {
  #[serde(rename="*")]
  All,
  Bool(bool),
  Vec(Vec<String>)
}
pub(crate) type Val=Option<Value>;


pub(in crate::ser::config) trait Parse {
  fn parse(&self,option: &str)-> Box<str>;
}

impl Parse for Val {
  fn parse(&self,option: &str)-> Box<str> {
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
  fn parse(&self,option: &str)-> Box<str> {
    match self {
      Some(val)=> val.parse(option),
      _=> "".into()
    }
  }
}





impl Parse for bool {
  fn parse(&self,option: &str)-> Box<str> {
    match self {
      true=> option,
      _=> ""
    }.into()
  }
}

impl Parse for u128 {
  fn parse(&self,option: &str)-> Box<str> {
    format!("{option} {self}").into_boxed_str()
  }
}



impl Parse for Box<Path> {
  fn parse(&self,option: &str)-> Box<str> {
    format!("{option}={}",self.display()).into_boxed_str()
  }
}

impl Parse for Box<[Box<str>]> {
  fn parse(&self,option: &str)-> Box<str> {
    format!("{option}=\"{}\"",self.join(",")).into_boxed_str()
  }
}

impl Parse for Box<str> {
  fn parse(&self,option: &str)-> Box<str> {
    format!("{option} {}",&self).into_boxed_str()
  }
}



#[cfg(test)]
mod tests {
  use crate::ser::config::ToArgs;


  #[tokio::test]
  async fn xd() {
    crate::config::Config::default().save("./proton-config.json").await.unwrap()
  }

  #[tokio::test]
  async fn read() {
    println!("{:#?}",crate::config::Config::find_config_file().await.unwrap().to_flags())
  }
}
