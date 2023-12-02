mod ser;
use ser::*;


use tokio::*;
use std::{
  env,
  collections::LinkedList,
  ffi::OsString
};


#[main]
async fn main() {
  let mut args: Box<[OsString]>=env::args_os().collect();
  let (i,rest)=extract(&mut args);
  let args=Operation::new(&(*args)[0..i]);
  
  println!("{:?}",&args);
  
  args.init().await.unwrap()
}

fn extract(args: &mut [OsString])-> (usize,LinkedList<&OsString>) {
  let mut deno_args=LinkedList::<&OsString>::new();
  let mut i=args.len();
  
  for str in args.iter().rev() {
    i-=1;
    if str.eq("--") {
      return (i,deno_args);
    }

    deno_args.push_front(str);
  }

  (i,deno_args)
}
