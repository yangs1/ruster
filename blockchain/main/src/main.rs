 use core::blockchain;
use std::{time::Duration, thread};
fn main() {
    // println!("Hello, world!");

     let mut bc = blockchain::BlockChain::new_blockchain();

     println!("start mining...");
     bc.add_block(String::from("a -> b: 5 btc."));
     thread::sleep(Duration::from_secs(2));

     println!("start mining...");

     thread::sleep(Duration::from_secs(2));

     bc.add_block(String::from("b -> a: 2 btc."));

     for b in bc.blocks{
         println!("################################");
         println!("{:#?}", b);
     }
}
