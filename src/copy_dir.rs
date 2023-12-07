use tokio::*;


use std::{
  collections::LinkedList,
  path::Path
};




/// copies a directory to specified `dest` path recursively. (actually iteratively)
/// 
/// `exceptions` is actually a regex expression that decides whether a file/directory should be copied.
/// # Example
/// ```rs
/// copy_dir_all("./test/repo","./test/xd",".git*").await?;
/// ```
#[allow(unused)]
pub async fn copy_dir_all<P: AsRef<Path>>(from: P,to: P,exceptions: &str)-> io::Result<()> {
  let except=regex::Regex::new(exceptions).unwrap();
  let mut queue=LinkedList::from_iter([(from.as_ref().to_owned(),to.as_ref().to_owned())]);

  while let Some((src,dest))=queue.pop_front() {
    fs::create_dir_all(&dest).await?;
    let mut iter=fs::read_dir(src).await?;

    while let Some(entry)=iter.next_entry().await? {
      if except.is_match(entry.path().to_str().unwrap_or_default()) {
        continue;
      }

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
    super::copy_dir_all("./test/repo","./test/xd",".git*").await.unwrap()
  }
}

