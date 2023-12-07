
use tokio::*;
use io::Error;
use crate::TEMPLATES;


use std::{
  env,
  path::Path
};

use requestty::{
  Question,
  prompt_one
};

use crossterm::style::{
  Color,
  style,
  Stylize
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

  let msg=format!("{}: {} is not an empty directory. Do you want to continue?",style("warning").with(Color::Yellow),path.display());
  let prompt=confirm(&msg,false);

  match prompt {
    true=> Ok(()),
    false=> Err(io::Error::new(
      io::ErrorKind::AlreadyExists,
      format!("{} is not an empty directory",path.display()).as_str()
    ))
  }
}

pub(crate) async fn ensure_dir<P: AsRef<Path>>(path: P)-> io::Result<()> {
  match fs::try_exists(&path).await {
    Err(_)=> fs::create_dir_all(path).await,
    _=> Ok(()),
  }
}


/// colors as string.
fn rgb((name,r,g,b): (&str,u8,u8,u8))-> String {
  style(name).with(Color::Rgb { r,g,b }).to_string()
}

pub(crate) fn ensure_template<'a>(template: Option<String>)-> String {
  if let Some(template)=template {
    return template;
  }

  let q=Question::select("Choose a Template")
  .choices(TEMPLATES.map(rgb))
  .build();

  let prompt=prompt_one(q).unwrap().try_into_list_item().unwrap();
  
  TEMPLATES[prompt.index].0.to_lowercase()
}

pub fn ensure_lang<'a>(ts: Option<bool>)-> &'a str {
  if let Some(ts)=ts {
    return lang(ts);
  }

  let q=Question::select("Choose your language")
  .choices([
    rgb(("TypeScript",0x2d,0x79,0xc7)),
    rgb(("JavaScript",0xff,0xff,0x0))
  ])
  .build();
  let prompt=prompt_one(q).unwrap().try_into_list_item().unwrap();

  lang(prompt.index!=0)
}

fn lang<'a>(ts: bool)-> &'a str {
  match ts {
    true=> "ts",
    false=> "js",
  }
}


pub(crate) fn url(template: &str,lang: &str)-> String {
  format!("https://github.com/proton-xd-templates/{template}-template-{lang}")
}


pub(crate) async fn clone_repo<P: AsRef<Path>>(url: &str,into: P)-> io::Result<()> {
  let temp=env::temp_dir();

  match git2::Repository::clone_recurse(url,&temp) {
    Ok(_)=> {
      //switching to temp dir
      env::set_current_dir(&temp)?;
      //cleaning up git-repo stuff as there may already be a repo in `into`
      fs::remove_dir_all("./git").await?;
      fs::remove_file(".gitignore").await?;

      // copy contents of `temp` to `into`
      //fs::copy(...).await?;

      //switching back to into
      env::set_current_dir(into)
    },
    Err(err)=> Err(Error::from_raw_os_error(err.raw_code())),
  }
}

