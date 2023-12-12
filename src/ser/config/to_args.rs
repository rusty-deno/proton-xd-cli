
use std::{
  collections::LinkedList,
  ffi::OsStr
};



pub(crate) trait ToArgs {
  fn to_flags(&self)-> LinkedList<Option<Box<str>>>;

  fn parse_args<I: FromIterator<Box<OsStr>>>(&self)-> I {
    self.to_flags().into_iter().filter_map(|arg| Some(OsStr::new(arg?.as_ref()).into())).collect()
  }
}

