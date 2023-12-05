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
    let q=requestty::Question::input("Project name").default("my-app").build();


    println!("{:#?}",requestty::prompt_one(q).unwrap().as_string().unwrap());
  }
}

