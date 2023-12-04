
use tokio::*;
use clap::Parser;
use std::path::PathBuf;




#[derive(Parser,Debug)]
pub struct Init {
  path: Option<PathBuf>,
  #[arg(short,long)]
  template: Option<Box<str>>,
  #[arg(long)]
  ts: Option<bool>,
  #[arg(long)]
  js: Option<bool>
}

impl Init {
  pub async fn init(self)-> io::Result<()> {
    unimplemented!()
  }
}




