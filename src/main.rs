mod ser;
pub(crate) mod api;
use ser::*;


#[tokio::main]
async fn main() {
  let args=Operation::new();


  println!("{:?}",&args);
  args.spawn().await.unwrap()
}



#[cfg(test)]
mod tests {
  use requestty::{
    Question,
    prompt_one
  };
  
  use crossterm::style::{
    style,
    Color,
    Stylize
  };

use crate::ser::TEMPLATES;

  fn rgb((name,r,g,b): (&str,u8,u8,u8))-> String {
    style(name).with(Color::Rgb { r,g,b }).to_string()
  }

  #[tokio::test]
  async fn xd() {
    let q=Question::select("Choose").choices(TEMPLATES.map(rgb)).build();

    println!("{:#?}",style(&prompt_one(q).unwrap().as_list_item().unwrap().text).content());
  }
}

