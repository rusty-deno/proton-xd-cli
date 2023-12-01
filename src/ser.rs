use clap::Parser;


#[derive(Parser,Debug)]
#[command(author,version,about,long_about=None)]
pub enum Operation {
  Build(Build)
}

impl Operation {
  pub fn new()-> Self {
    Self::parse()
  }
}



#[derive(Parser,Debug)]
pub struct Build {
  #[arg(short='r')]
  release: bool,
  #[arg(short='p')]
  path: String,
}

