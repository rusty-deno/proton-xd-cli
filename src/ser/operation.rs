
use tokio::*;
use clap::Parser;


use super::{
  build::Build,
  init::Init
};





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
    #[arg(long,default_value="false")]
    js: bool
  },
  Init(Init)
}

impl Operation {
  pub fn new()-> Self {
    Self::parse()
  }

  pub async fn spawn(self)-> io::Result<()> {
    match self {
      Operation::Build(builder)=> builder.build().await,
      Operation::Init(initializer)=> initializer.init().await,
      _=> todo!()
    }
  }
}






