
use tokio::*;
use clap::Parser;
use std::path::PathBuf;


use crate::api::*;
use super::build::Build;





#[derive(Parser,Debug)]
#[command(author,version,about,long_about=None)]
pub enum Operation {
  Build(Build),
  New {
    path: Box<str>,
    #[arg(short,long)]
    template: Option<Box<str>>,
    #[arg(long)]
    ts: bool,
    #[arg(long)]
    js: bool
  },
  Init {
    path: Option<PathBuf>,
    #[arg(short,long)]
    template: Option<Box<str>>,
    #[arg(long)]
    ts: bool,
    #[arg(long)]
    js: bool
  }
}

impl Operation {
  pub fn new()-> Self {
    Self::parse()
  }

  pub async fn spawn(self)-> io::Result<()> {
    match self {
      Operation::Build(build)=> build.build().await,
      Operation::Init { path,template,ts,.. }=> clone_repo(path,template,ts).await,
      _=> todo!()
    }
  }
}






