
use std::collections::LinkedList;
use super::Str;



pub(crate) trait ToArgs {
  fn to_flags(&self)-> LinkedList<Option<Box<str>>>;

  fn parse_args<I: FromIterator<Str>>(&self)-> I {
    self.to_flags().into_iter().filter_map(|arg| arg).collect()
  }
}

