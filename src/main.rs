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
  use crate::api::confirm;



  #[tokio::test]
  async fn xd() {
    println!("{}",confirm("msg",false).await.unwrap())
  }
}

