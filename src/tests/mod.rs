mod main;


use std::env;
use tokio::*;
use tokio::test;


#[test]
async fn parent_of_root() {
  _parent_of_root().await.unwrap();
}

async fn _parent_of_root()-> io::Result<()> {
  env::set_current_dir("/")?;
  env::set_current_dir("..")?;

  let mut iter=fs::read_dir(".").await?;

  while let Some(xd)=iter.next_entry().await? {
    println!("{}",xd.path().display())
  }

  Ok(())
}


#[test]
async fn size() {
  std::rc::Rc::new(69u8);
}

