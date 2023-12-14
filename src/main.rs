mod ser;
mod copy_dir;
pub(crate) mod api;
mod tests;


use ser::*;

#[tokio::main]
async fn main() {
  let args=Operation::new();
  args.spawn().await.unwrap()
}


