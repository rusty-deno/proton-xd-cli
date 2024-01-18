
use tokio::*;
use clap::Parser;


use super::{
  build::Build,
  init::Init,
  new::New,
  dev::Dev, Operation
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
}

impl Operation for App {
  async fn run(self)-> io::Result<()> {
    match self {
      App::Build(app)=> app.run().await,
      App::New(app)=> app.run().await,
      App::Init(app)=> app.run().await,
      App::Dev(app)=> app.run().await,
    }
  }
}






