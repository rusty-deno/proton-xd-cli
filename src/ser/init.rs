
use tokio::*;
use clap::Parser;
use std::path::PathBuf;


use requestty::{
  Question,
  prompt_one,
  Result as Res
};

use crate::api::*;



#[derive(Parser,Debug)]
pub struct Init {
  #[arg(short,long)]
  template: Option<Box<str>>,
  #[arg(long)]
  ts: Option<bool>,
  #[arg(long)]
  js: Option<bool>
}

impl Init {
  pub async fn init(self)-> io::Result<()> {
    ensure_fresh_dir("./").await?;

    let url=url(&self.template.unwrap(),self.ts.unwrap_or(false));
    clone_repo(&url,"./")
  }
}


fn _ensure_path(path: Option<PathBuf>)-> Res<PathBuf> {
  match path {
    Some(path)=> Ok(path),
    None=> {
      let question=Question::input("Project name").default("my-app").build();
      
      Ok(prompt_one(question)?.as_string().unwrap().into())
    }
  }
}



