
use clap::Parser;
use super::{Operation, config::Config};



#[derive(Debug,Parser)]
pub struct Dev;

impl Operation for Dev {
  async fn run(self)-> std::io::Result<()> {
    let _config=Config::find_config_file().await?;


    Ok(())
  }
}

