use clap::Parser;
use std::path::PathBuf;
use serde::Deserialize;


#[derive(Debug,Parser)]
pub(crate) struct Build {
  #[arg(long)]
  dir: Option<PathBuf>,
}



#[derive(Deserialize)]
enum Permission {
  Bool(bool),
  Vec(Box<[String]>)
}
type Perm=Option<Permission>;


#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Config {
  no_config: Perm,
  unstable: bool,
}







#[test]
fn xd() {
  let _xd: Config=serde_json::from_str(r#"{

  }"#).unwrap();
  

}
