use tokio::*;
use clap::Parser;


use std::{
  path::Path,
  ffi::OsStr
};

use super::{
  config::{
    Config,
    ToArgs,
    Str
  },
  MAIN
};


#[derive(Debug,Parser)]
pub struct Build {
  #[arg(long,default_value="build")]
  dir: Box<Path>,
}

impl Build {
  pub async fn build(self)-> io::Result<()> {
    let config=Config::find_config_file().await?;
    let args=config.to_flags().into_iter().filter_map(to_boxed_os_str);

    let mut cmd=process::Command::new("deno");
    cmd.arg("compile");
    cmd.args(args);
    cmd.arg(format!("{}.{}",config.main.unwrap_or(MAIN.into()),config.language.unwrap_or_default().extension()));

    cmd.spawn()?.wait().await?;

    Ok(())
  }
}

fn to_boxed_os_str(str: Option<Str>)-> Option<Box<OsStr>> {
  Some(OsStr::new(str?.as_ref()).into())
}

