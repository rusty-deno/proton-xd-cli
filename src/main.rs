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
  #[tokio::test]
  async fn xd() {
    let q=requestty::Question::confirm("this aint an empty directory!").default(false).build();


    println!("{:#?}",requestty::prompt_one(q).unwrap().as_bool().unwrap());
  }
}

