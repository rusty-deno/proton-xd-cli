
use tokio::*;
use std::{
  env,
  path::PathBuf
};




pub async fn cwd()-> io::Result<String> {
  let cwd=env::current_dir()?;

  fs::read_to_string(cwd.join("proton-config.ts")).await
}

pub async fn ensure_empty_dir(path: &PathBuf)-> io::Result<()> {
  match fs::read_dir(path).await?.next_entry().await? {
    Some(path)=> panic!("{path:?} is not an empty directory!"),
    None=> Ok(()),
  }
}

pub async fn ensure_dir(path: &PathBuf)-> io::Result<()> {
  if fs::try_exists(path).await? {
    return Ok(());
  }
  fs::create_dir(path).await
}






