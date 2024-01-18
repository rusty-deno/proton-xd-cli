use std::{
  path::Path,
  collections::LinkedList
};

use serde::{
  Deserialize,
  Serialize
};

use super::{
  ToArgs,
  Parse
};




deno_option_type! {
  CompilerOptions: {
    pub include: Option<Box<Path>> = None,
    pub output: Option<Box<Path>> = None,
    pub target: Option<Box<str>> = None,
    pub no_terminal: bool = true
  }
}


impl ToArgs for CompilerOptions {
  fn to_flags(&self)-> LinkedList<Option<Box<str>>> {
    LinkedList::from_iter([
      self.no_check.parse("--no-check"),
      self.import_map.parse("--import-map"),
      self.no_remote.parse("--no-remote"),
      self.no_npm.parse("--no-npm"),
      self.node_modules_dir.parse("--node-modules-dir"),
      self.vendor.parse("--vendor"),
      self.config.parse("--config"),
      self.reload.parse("--reload"),
      self.lock.parse("--lock"),
      self.lock_write.parse("--lock-write"),
      self.no_lock.parse("--no-lock"),
      self.cert.parse("--cert"),
      self.quiet.parse("--quiet"),
      self.unsafely_ignore_certificate_errors.parse("--unsafely-ignore-certificate-errors"),
      self.no_prompt.parse("--no-prompt"),
      self.catch_only.parse("--catch-only"),
      self.location.parse("--location"),
      self.v8_flags.parse("--v8-flags"),
      self.seed.parse("--seed"),
      self.check.parse("--check"),
      self.include.parse("--include"),
      self.output.parse("--output"),
      self.target.parse("--targer"),
      self.no_terminal.parse("--no-terminal"),
      self.env.parse("--env")
    ])
  }
}

