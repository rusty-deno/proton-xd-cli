use tokio::*;


use std::{
  collections::LinkedList,
  path::Path
};




/// Copies a directory to specified `dest` path recursively. (actually iteratively)
/// 
/// `exceptions` is actually a regex expression that decides whether a file/directory should be copied.
/// 
/// NOTE: It doesn't care about symlinks.
/// # Example
/// ```rs
/// copy_dir_all("./test/repo","./test/xd",".git*").await?;
/// ```
pub(crate) async fn copy_dir_all<F: AsRef<Path>,T: AsRef<Path>>(from: F,to: T,exceptions: &str)-> io::Result<()> {
  let except=regex::Regex::new(exceptions).unwrap();
  let mut queue=LinkedList::<(Box<Path>,Box<Path>)>::from_iter([(from.as_ref().into(),to.as_ref().into())]);

  while let Some((src,dest))=queue.pop_front() {
    fs::create_dir_all(&dest).await?;
    let mut iter=fs::read_dir(src).await?;

    while let Some(entry)=iter.next_entry().await? {
      if except.is_match(entry.path().to_str().unwrap_or_default()) {
        continue;
      }

      let entry_type=entry.file_type().await?;
      let entry_dest_path=dest.join(entry.file_name()).into_boxed_path();

      match entry_type.is_file() {
        true=> {
          tokio::spawn(fs::copy(entry.path(),entry_dest_path)).await??;
        },
        _=> queue.push_back((entry.path().into_boxed_path(),entry_dest_path))
      }
    }
  }

  Ok(())
}


pub(crate) struct TempDir {
  path: Box<Path>
}

impl Drop for TempDir {
  fn drop(&mut self) {
    let path=std::mem::replace(&mut self.path,Path::new("").into());
    tokio::task::spawn(fs::remove_dir_all(path));
  }
}

impl TempDir {

  pub(crate) async fn new()-> io::Result<Self> {
    let path=Path::new("");
    fs::create_dir_all(path).await?;
    
    Ok(Self {
      path: path.into()
    })
  }

  pub(crate) fn path(&self)-> &Path {
    &self.path
  }

  pub(crate) async fn move_to<P: AsRef<Path>>(self,to: P,exceptions: &str)-> io::Result<()> {
    copy_dir_all(&self.path,to,exceptions).await
  }
}


