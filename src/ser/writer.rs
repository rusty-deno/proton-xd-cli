#![allow(dead_code)]
use std::{
  collections::LinkedList,
  io::{
    Write,
    self
  },
};

pub struct Writer {
  stream: LinkedList<u8>
}

impl Writer {
  pub fn new()-> Self {
    Self {
      stream: Default::default()
    }
  }
  
  pub fn to_vec(self)-> Vec<u8> {
    self.stream.into_iter().collect()
  }

  pub fn to_string(self)-> String {
    unsafe { String::from_utf8_unchecked(self.to_vec()) }
  }
}

impl Write for Writer {
  fn write(&mut self,buf: &[u8])-> io::Result<usize> {
    let len=self.stream.len();

    for byte in buf {
      self.stream.push_back(*byte);
    }

    Ok(self.stream.len()-len)
  }

  fn flush(&mut self)-> io::Result<()> {
    Ok(())
  }
}














