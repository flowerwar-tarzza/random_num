use random::memo::{self,*};
use std::{env,env::Args};

fn main() {
    let mut args:Args = env::args();
    if args.len() < 2 {
        println!("input file name too.!");
        return ;
    }
    println!("{}",args.len());

    args.next(); // discard run file
    let file_name = args.next().unwrap();

    let book = make_book(&file_name);
    let mut memo_manager = MemoManager::build(book,file_name);

    memo_manager.run();
}
