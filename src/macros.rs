
macro_rules! deno_option_type {
  ($name:ident: {
    $(
      $vis:vis $field:ident: $typ:ty=$default:expr
    ),*
  })=> {
    #[derive(Deserialize,Serialize,Debug)]
    #[serde(rename_all="kebab-case")]
    pub struct $name {
      pub(crate) no_check: Option<bool>,
      pub(crate) import_map: Option<Box<Path>>,
      pub(crate) no_remote: Option<bool>,
      pub(crate) no_npm: Option<bool>,
      pub(crate) node_modules_dir: Option<Box<[Box<str>]>>,
      pub(crate) vendor: Option<bool>,
      pub(crate) config: Option<Box<Path>>,
      pub(crate) reload: Option<Box<[Box<str>]>>,
      pub(crate) lock: Option<Box<Path>>,
      pub(crate) lock_write: Option<bool>,
      pub(crate) no_lock: Option<bool>,
      pub(crate) cert: Option<Box<Path>>,
      pub(crate) quiet: Option<bool>,
      pub(crate) unsafely_ignore_certificate_errors: Option<Box<[Box<str>]>>,
      pub(crate) no_prompt: bool,
      pub(crate) catch_only: Option<bool>,
      pub(crate) location: Option<Box<Path>>,
      pub(crate) v8_flags: Option<Box<[Box<str>]>>,
      pub(crate) seed: Option<u128>,
      pub(crate) check: Option<bool>,
      pub(crate) env: Option<Box<Path>>,
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
  };
}

