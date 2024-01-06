use random::memo::*;
use std::{env,env::Args};

fn main() {
    let mut args:Args = env::args();
    println!("{}",args.len());

    args.next(); // discard run file
    let file_name = args.next().unwrap();

    let book = make_book(file_name);
    let mut memo_manager = MemoManager::build(book);

    println!("시작부터 : 볼 갯수");
    //memo_manager.display_memo_key_control();
    memo_manager.display_memo_async_stdin();
}
