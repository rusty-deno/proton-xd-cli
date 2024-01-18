
use clap::Parser;

use super::Operation;



#[derive(Debug,Parser)]
pub struct Dev;

impl Operation for Dev {
  async fn run(self)-> std::io::Result<()> {
    todo!()
  }
}

