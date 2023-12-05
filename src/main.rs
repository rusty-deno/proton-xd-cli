mod ser;
pub mod api;


use ser::*;
use tokio::*;



#[main]
async fn main() {
  let args=Operation::new();
  
  println!("{:?}",&args);
  
  args.spawn().await.unwrap()


}



#[cfg(test)]
mod tests {
  use crate::api::ensure_fresh_dir;


  #[tokio::test]
  async fn xd() {
    ensure_fresh_dir("./test").await.unwrap()
  }
}

