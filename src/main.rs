mod ser;
use std::path::PathBuf;

use ser::*;


use tokio::*;


#[main]
async fn main() {
  let args=Operation::new();
  
  println!("{:?}",&args);

  match args {
    Operation::Build { dir }=> build(dir).await.unwrap(),
    _=> todo!()
  }
}

async fn build(dir: PathBuf)-> io::Result<()> {
  ensure_dir(&dir).await?;

  let mut process=process::Command::new("deno");
  process.arg("compile")
  .args(["--no-prompt","-o",dir.join("xd").to_str().unwrap(),])
  .arg("./proton-xd-src/main.ts");

  std::process::exit(process.spawn()?.wait().await?.code().unwrap_or_default())
}

async fn ensure_dir(path: &PathBuf)-> io::Result<()> {
  if fs::try_exists(path).await? {
    return Ok(());
  }
  fs::create_dir(path).await
}


