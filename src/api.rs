
use tokio::*;
use io::Error;
use std::path::Path;
use crate::TEMPLATES;

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

  let msg=format!("{}: {path:?} is not an empty directory. Do you want to continue?",style("warning").with(Color::Yellow));
  let prompt=confirm(&msg,false);

  match prompt {
    true=> Ok(()),
    false=> Err(io::Error::new(
      io::ErrorKind::AlreadyExists,
      format!("{path:?} is not an empty directory").as_str()
    ))
  }
}

pub(crate) async fn ensure_dir<P: AsRef<Path>>(path: P)-> io::Result<()> {
  if fs::try_exists(&path).await? {
    return Ok(());
  }
  fs::create_dir_all(path).await
}


/// colors as string.
fn rgb((name,r,g,b): (&str,u8,u8,u8))-> String {
  style(name).with((r,g,b).into()).to_string()
}

pub(crate) fn ensure_template<'a>(template: Option<String>)-> String {
  if let Some(template)=template {
    return template;
  }

  let q=Question::select("Choose a Template")
  .choices(TEMPLATES.map(rgb))
  .build();

  let prompt=prompt_one(q).unwrap().try_into_list_item().unwrap();
  
  TEMPLATES[prompt.index].0.to_owned().to_lowercase()
}

pub fn ensure_lang<'a>(ts: Option<bool>)-> &'a str {
  if let Some(ts)=ts {
    return lang(ts);
  }
  // 0x2d79c7

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
  match git2::Repository::clone_recurse(url,into) {
    Ok(_)=> {
      fs::remove_dir("./git").await?;
      fs::remove_file(".gitignore").await
    },
    Err(err)=> Err(Error::from_raw_os_error(err.raw_code())),
  }
}

