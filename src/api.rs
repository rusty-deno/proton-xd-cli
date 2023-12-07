
use tokio::*;

use std::{
  env,
  path::{
    Path,
    PathBuf
  }
};

use crossterm::style::{
  Color,
  style,
  Stylize
};

use requestty::{
  Question,
  prompt_one
};


pub(crate) fn confirm(msg: &str,default: bool)-> bool {
  let q=Question::confirm(msg).default(default).build();
  
  match prompt_one(q) {
    Ok(res)=> res.as_bool().unwrap(),
    _=> default
  }
}


pub(crate) async fn ensure_fresh_dir<P: AsRef<Path>>(path: P)-> io::Result<()> {
  let path=path.as_ref();

  if fs::read_dir(path).await?.next_entry().await?.is_none() {
    return Ok(());
  }

  let msg=format!("{}: {path:?} is not an empty directory. Do you want to continue?",style("warning").with(Color::Yellow));
  let prompt=confirm(&msg,false);

  match prompt {
    true=> Ok(()),
    false=> Err(io::Error::new(
      io::ErrorKind::AlreadyExists,
      format!("A proton-xd project already exists in {path:?}").as_str()
    ))
  }
}

pub async fn ensure_dir<P: AsRef<Path>>(path: P)-> io::Result<()> {
  if fs::try_exists(&path).await? {
    return Ok(());
  }
  fs::create_dir_all(path).await
}


pub async fn clone_repo(path: Option<PathBuf>,_template: Option<Box<str>>,ts: bool)-> io::Result<()> {
  let path=path.unwrap_or(env::current_dir()?);
  ensure_fresh_dir(&path).await?;

  let _url=format!("https://github.com/kakashi-69-xd/proton-xd-templates/{}/{}",lang(ts),"next");



  Ok(())
}

pub fn lang<'a>(ts: bool)-> &'a str {
  match ts {
    true=> "ts",
    false=> "js",
  }
}




