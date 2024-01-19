
macro_rules! deno_option_type {
  ($name:ident {
    $(
      #[flag=$flag:literal]
      $vis:vis $field:ident: $typ:ty=$default:expr
    ),*
  })=> {
    #[derive(serde::Deserialize,serde::Serialize,Debug)]
    #[serde(rename_all="kebab-case")]
    pub struct $name {
      pub(crate) no_check: Option<bool>,
      pub(crate) import_map: Option<Box<std::path::Path>>,
      pub(crate) no_remote: Option<bool>,
      pub(crate) no_npm: Option<bool>,
      pub(crate) node_modules_dir: Option<Box<[Box<str>]>>,
      pub(crate) vendor: Option<bool>,
      pub(crate) config: Option<Box<std::path::Path>>,
      pub(crate) reload: Option<Box<[Box<str>]>>,
      pub(crate) lock: Option<Box<std::path::Path>>,
      pub(crate) lock_write: Option<bool>,
      pub(crate) no_lock: Option<bool>,
      pub(crate) cert: Option<Box<std::path::Path>>,
      pub(crate) quiet: Option<bool>,
      pub(crate) unsafely_ignore_certificate_errors: Option<Box<[Box<str>]>>,
      pub(crate) no_prompt: bool,
      pub(crate) catch_only: Option<bool>,
      pub(crate) location: Option<Box<std::path::Path>>,
      pub(crate) v8_flags: Option<Box<[Box<str>]>>,
      pub(crate) seed: Option<u128>,
      pub(crate) check: Option<bool>,
      pub(crate) env: Option<Box<std::path::Path>>,
      $(
        $vis $field: $typ,
      )*
    }


    impl Default for $name {
      fn default()-> Self {
        Self {
          no_prompt: true,
          // none
          no_check: None,
          catch_only: None,
          import_map: None,
          no_remote: None,
          no_npm: None,
          node_modules_dir: None,
          vendor: None,
          config: None,
          reload: None,
          lock: None,
          lock_write: None,
          no_lock: None,
          cert: None,
          quiet: None,
          unsafely_ignore_certificate_errors: None,
          location: None,
          v8_flags: None,
          seed: None,
          check: None,
          env: None,
          $(
            $field: $default,
          )*
        }
      }
    }

    impl crate::config::ToArgs for $name {
      fn to_flags(&self)-> std::collections::LinkedList<Option<Box<str>>> {
        use crate::config::Parse;
        std::collections::LinkedList::from_iter([
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
          self.env.parse("--env"),
          $(
            self.$field.parse($flag),
          )*
        ])
      }
    }
  };
}

