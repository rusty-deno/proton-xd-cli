use tokio::*;
use clap::Parser;
use std::path::Path;

use super::{
  config::Config,
  Operation
};


#[derive(Debug,Parser)]
pub struct Build {
  #[arg(short,long,default_value="build")]
  out: Box<Path>,
}

impl Operation for Build {
  async fn run(self)-> io::Result<()> {
    let mut config=Config::find_config_file().await?;
    config.compiler_options.output.get_or_insert(self.out);

    command! {
      cmd: "compile",
      config: config
    }
  }
}

