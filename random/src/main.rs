use random::memo::*;
use std::{env,env::Args};

fn main() {
    let mut args:Args = env::args();
    println!("{}",args.len());
    let mut file_name:String = String::new();
    let mut start:usize = 1;
    let mut amount:usize = 1;

    args.next();
    println!("args len:{}",args.len());
    match args.len() {
        1 => {file_name = args.next().unwrap();},
        2 => { file_name = args.next().unwrap();
            start = args.next().unwrap().parse().unwrap(); },
        3 => { file_name = args.next().unwrap();
            start = args.next().unwrap().parse().unwrap();
            amount = args.next().unwrap().parse().unwrap(); },
        _ => {println!(" ex)file_name start amount!"); return;},
    }

    for argument in args {
        println!("{argument}");
    }

    let book = make_book(&file_name);
    let memo_manager = MemoManager::build(book);

    println!("시작부터 : 볼 갯수");
    memo_manager.display_memo(MemoShowRange::Select(start,amount),MemoShowMethod::WORD_MEAN_EXAMPLE);
    //memo_manager.display_memo(MemoShowRange::All);
}
