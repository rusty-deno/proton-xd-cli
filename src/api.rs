
use tokio::*;
use std::{
  env,
  path::PathBuf
};

use crossterm::style::{
  Color,
  style,
  Stylize
};


pub async fn ensure_empty_dir(path: &PathBuf)-> io::Result<()> {
  match fs::read_dir(path).await?.next_entry().await? {
    None=> Ok(()),
    Some(path)=> {
      let _msg=format!("{}: {path:?} is not an empty directory!",style("warning").with(Color::Yellow));
      

      todo!()
    }
  }
}

pub async fn ensure_dir(path: &PathBuf)-> io::Result<()> {
  if fs::try_exists(path).await? {
    return Ok(());
  }
  fs::create_dir(path).await
}


pub async fn clone_repo(path: Option<PathBuf>,_template: Option<Box<str>>,ts: bool)-> io::Result<()> {
  let path=path.unwrap_or(env::current_dir()?);
  ensure_empty_dir(&path).await?;

  let _url=format!("https://github.com/kakashi-69-xd/proton-xd-templates/{}/{}",lang(ts),"next");



  Ok(())
}

pub fn lang<'a>(ts: bool)-> &'a str {
  match ts {
    true=> "ts",
    false=> "js",
  }
}




