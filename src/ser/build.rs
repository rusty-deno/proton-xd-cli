use tokio::*;
use clap::Parser;


use std::{
  path::Path,
  ffi::OsStr
};

use super::config::{
  Config,
  ToArgs
};


#[derive(Debug,Parser)]
pub struct Build {
  #[arg(long,default_value="build")]
  dir: Box<Path>,
}

impl Build {
  pub async fn build(self)-> io::Result<()> {
    // fetches config file and sets `current_dir` to its directory.
    let _config=Config::find_config_file().await?;
  
    process::Command::new("program").args(_config.parse_args::<std::collections::LinkedList<Box<OsStr>>>());



    Ok(())
  }
}







