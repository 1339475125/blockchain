use core::blockchain;
use std::thread;
use std::time::Duration;


fn main() {
    let mut bc = blockchain::BlockChain::new_block_chain();

    println!("starting mining .....");
    thread::sleep(Duration::from_micros(10));
    bc.add_block(String::from("a -> b: 5btc"));
    println!("produce a block!");

    println!("starting mining .....");
    thread::sleep(Duration::from_micros(10));
    bc.add_block("c -> d: 1btc".to_string());
    println!("produce a block!");
    for b in bc.blocks{
        println!("++++++++++++++");
        println!("{:#?}", b);
        println!("");
    }
}
