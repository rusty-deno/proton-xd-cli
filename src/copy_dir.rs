use tokio::*;


use std::{
  collections::LinkedList,
  path::Path
};




#[allow(unused)]
pub async fn copy_dir_all<P: AsRef<Path>>(from: P,to: P)-> io::Result<()> {
  let mut queue=LinkedList::from_iter([(from.as_ref().to_owned(),to.as_ref().to_owned())]);

  while let Some((src,dest))=queue.pop_front() {
    fs::create_dir_all(&dest).await?;
    let mut iter=fs::read_dir(src).await?;

    while let Some(entry)=iter.next_entry().await? {
      let entry_type=entry.file_type().await?;
      let entry_dest_path=dest.join(entry.file_name());

      match entry_type.is_file() {
        true=> {
          tokio::spawn(fs::copy(entry.path(),entry_dest_path)).await??;
        },
        _=> queue.push_back((entry.path(),entry_dest_path))
      }
    }
  }

  Ok(())
}




#[cfg(test)]
mod tests {
  use tokio::test;
  #[test]
  async fn xd() {
    super::copy_dir_all("./test/repo","./test/xd").await.unwrap()
  }
}

