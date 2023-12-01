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
    #[arg(default_value="./")]
    path: PathBuf,
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






// match value.as_ref() {
//   "next"=> Next,
//   "react"=> React,
//   "remix"=> Remix,
//   "ruck"=> Ruck,
//   "svelte"=> Svelte,
//   "sanilla"=> Vanilla,
//   "vue"=> Vue,
//   val=> panic!("{val} is not a valid template")
// }

