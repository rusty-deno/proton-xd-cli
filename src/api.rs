
use tokio::*;
use crate::ser::config::CONFIG_FILE_NAME;

use std::{
  env,
  path::PathBuf
};

use crossterm::style::{
  Color,
  style,
  Stylize
};

use prompts::{
  Prompt,
  confirm::ConfirmPrompt
};



pub async fn ensure_fresh_dir(path: &PathBuf)-> io::Result<()> {
  match fs::try_exists(path.join(CONFIG_FILE_NAME)).await? {
    false=> Ok(()),
    true=> {
      let msg=format!("{}: {path:?} is not an empty directory!",style("warning").with(Color::Yellow));
      let mut prompt=ConfirmPrompt::new(&msg).set_initial(false);

      match prompt.run().await.unwrap().unwrap() {
        false=> std::process::exit(0),
        true=> {
          fs::remove_dir_all(path).await.unwrap();
          fs::create_dir_all(path).await
        },
      }
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




