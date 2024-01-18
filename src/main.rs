mod operation;
pub(crate) mod api;
mod tests;


use operation::*;

#[tokio::main]
async fn main() {
  let args=App::new();
  args.spawn().await.unwrap()
}


