
use tokio::*;
use clap::Parser;


use super::{
  build::Build,
  init::Init,
  new::New
};





#[derive(Parser,Debug)]
#[command(author,version,about,long_about=None)]
pub enum Operation {
  Build(Build),
  New(New),
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
      Operation::New(initilizer)=> initilizer.init().await
    }
  }
}






