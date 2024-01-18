
use tokio::*;
use clap::Parser;


use super::{
  build::Build,
  init::Init,
  new::New,
  dev::Dev
};


#[derive(Parser,Debug)]
#[command(author,version,about,long_about=None)]
pub enum App {
  Build(Build),
  New(New),
  Init(Init),
  Dev(Dev)
}

impl App {
  pub fn new()-> Self {
    Self::parse()
  }

  pub async fn spawn(self)-> io::Result<()> {
    match self {
      App::Build(builder)=> builder.build().await,
      App::Init(initializer)=> initializer.init().await,
      App::New(initilizer)=> initilizer.init().await,
      _=> todo!()
    }
  }
}






