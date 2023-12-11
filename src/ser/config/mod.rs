mod compiler_options;
mod config;
mod to_args;

use compiler_options::*;
pub(crate) use config::*;
pub(crate) use to_args::*;


use super::Writer;
use std::path::Path;



pub(in crate::ser::config) trait Parse {
  fn parse(&self,option: &str)-> Option<Box<str>>;
}

impl<S: Parse> Parse for Option<S> {
  fn parse(&self,option: &str)-> Option<Box<str>> {
    match self {
      Some(val)=> val.parse(option),
      _=> None
    }
  }
}




impl Parse for bool {
  fn parse(&self,option: &str)-> Option<Box<str>> {
    match self {
      true=> Some(option.into()),
      _=> None
    }
  }
}

impl Parse for u128 {
  fn parse(&self,option: &str)-> Option<Box<str>> {
    Some(format!("{option} {self}").into_boxed_str())
  }
}



impl Parse for Box<Path> {
  fn parse(&self,option: &str)-> Option<Box<str>> {
    Some(format!("{option}={}",self.display()).into_boxed_str())
  }
}

impl Parse for Box<[Box<str>]> {
  fn parse(&self,option: &str)-> Option<Box<str>> {
    Some(format!("{option}=\"{}\"",self.join(",")).into_boxed_str())
  }
}

impl Parse for Box<str> {
  fn parse(&self,option: &str)-> Option<Box<str>> {
    Some(format!("{option} {}",&self).into_boxed_str())
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
