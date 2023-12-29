use random::memo::*;
use std::{env,env::Args};

fn main() {
    let mut args:Args = env::args();
    println!("{}",args.len());
    let mut file_name:String = "look_again_test.csv".to_string();
    let mut start:usize = 1;
    let mut amount:usize = 1;

    if args.len() >= 4 {
        args.next();
        file_name = args.next().expect("파일명보기");
        start = args.next().expect("시작번호").parse().unwrap();
        amount = args.next().expect("볼양").parse().unwrap(); // 볼 양
    }

    for argument in args {
        println!("{argument}");
    }

    let book = make_book(&file_name);
    let memo_manager = MemoManager::build(book);

    println!("시작부터 : 볼 갯수");
    memo_manager.display_memo(MemoDisplayMethod::Select(start,amount));
}
