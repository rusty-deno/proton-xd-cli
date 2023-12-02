use clap::Parser;
use std::path::PathBuf;


#[derive(Debug,Parser)]
pub(crate) struct Build {
  #[arg(long)]
  dir: Option<PathBuf>,
}









