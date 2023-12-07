use tokio::*;
use clap::Parser;
use std::path::PathBuf;


#[derive(Debug,Parser)]
pub struct Build {
  #[arg(long,default_value="build")]
  dir: PathBuf,
}

impl Build {
  pub async fn build(self)-> io::Result<()> {
    // ensures that `path` exists.
    fs::create_dir_all(&self.dir).await?;

    let mut process=process::Command::new("deno");
    process.arg("compile")
    .args(["--no-prompt","-o",self.dir.join("xd").to_str().unwrap(),])
    .arg("./proton-xd-src/main.ts");

    std::process::exit(process.spawn()?.wait().await?.code().unwrap_or_default())
  }
}







