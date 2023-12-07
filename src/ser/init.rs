
use tokio::*;
use clap::Parser;
use crate::api::*;



#[derive(Parser,Debug)]
pub struct Init {
  #[arg(short,long)]
  template: Option<String>,
  #[arg(long)]
  ts: Option<bool>,
  #[arg(long)]
  js: Option<bool>
}

impl Init {
  pub async fn init(self)-> io::Result<()> {
    ensure_fresh_dir("./").await?;

    let url=url(&ensure_template(self.template),ensure_lang(self.ts));
    clone_repo(&url,"./")
  }
}




