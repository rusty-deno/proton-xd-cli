use std::path::PathBuf;
use clap::Parser;


#[derive(Parser,Debug)]
#[command(author,version,about,long_about=None)]
pub enum Operation {
  Build {
    #[arg(long="dir",default_value="build")]
    dir: PathBuf,
  },
  New {
    path: Box<str>,
    #[arg(short='t',long="template")]
    template: Option<Box<str>>,
    #[arg(long="ts")]
    ts: bool,
    #[arg(long="js")]
    js: bool
  },
  Init {
    path: Option<PathBuf>,
    #[arg(short='t',long="template")]
    template: Option<Box<str>>,
    #[arg(long="ts")]
    ts: bool,
    #[arg(long="js")]
    js: bool
  }
}

impl Operation {
  pub fn new()-> Self {
    Self::parse()
  }
}




