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
  #[arg(short,long,default_value="build")]
  out: Box<Path>,
}

impl Build {
  pub async fn build(self)-> io::Result<()> {
    let mut config=Config::find_config_file().await?;
    config.compiler_options.output.get_or_insert(self.out);
    let args=config.to_flags().into_iter().filter_map(to_boxed_os_str);

    let mut cmd=process::Command::new("deno");
    cmd.arg("compile");
    cmd.args(args);
    // # Example "./proton-src/main.ts"
    cmd.arg(format!("{}.{}",config.main.unwrap_or(MAIN.into()),config.language.unwrap_or_default().extension()));

    match cmd.spawn()?.wait().await {
      Err(err)=> Err(io::Error::new(err.kind(),"Some error occured while running deno.\nTry upgrading to the latest version of deno")),
      _=> Ok(()),
    }
  }
}

fn to_boxed_os_str(str: Option<Str>)-> Option<Box<OsStr>> {
  Some(OsStr::new(str?.as_ref()).into())
}

