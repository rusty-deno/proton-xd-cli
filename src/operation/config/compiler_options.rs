use std::path::Path;


deno_option_type! {
  CompilerOptions {
    #[flag="--include"]
    pub include: Option<Box<Path>> = None,
    #[flag="--output"]
    pub output: Option<Box<Path>> = None,
    #[flag="--target"]
    pub target: Option<Box<str>> = None,
    #[flag="--no-terminal"]
    pub no_terminal: bool = true
  }
}
