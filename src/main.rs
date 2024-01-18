
mod tests;
#[macro_use]
mod macros;
mod operation;
pub(crate) mod api;



use operation::*;

#[tokio::main]
async fn main() {
  let args=App::new();
  args.run().await.unwrap()
}


