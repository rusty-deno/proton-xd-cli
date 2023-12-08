use tokio::*;
use clap::Parser;
use std::path::PathBuf;
use super::config::Config;


#[derive(Debug,Parser)]
pub struct Build {
  #[arg(long,default_value="build")]
  dir: PathBuf,
}

impl Build {
  pub async fn build(self)-> io::Result<()> {
    // fetches config file and its path.
    let _config=Config::find_config_file().await?;




    Ok(())
  }
}







