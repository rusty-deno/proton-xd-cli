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
  use crossterm::style::{
    Color,
    style,
    Stylize
  };


  #[test]
  fn xd() {
    let msg=format!("{}: ./ is not an empty directory!",style("warning").with(Color::Yellow));

    println!("{msg}")
  }
}

