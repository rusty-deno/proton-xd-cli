mod ser;
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

  #[test]
  fn xd() {
    


  }
}

