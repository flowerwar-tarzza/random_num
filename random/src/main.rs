use random::memo::*;
use std::{env,env::Args};

fn main() {
    let mut args:Args = env::args();
    println!("{}",args.len());
    let mut file_name:String = String::new();
    let mut start:usize = 1;
    let mut amount:usize = 1;
    let mut method:MemoShowMethod = MemoShowMethod::WORD;

    args.next();
    println!("args len:{}",args.len());
    match args.len() {
        1 => {file_name = args.next().unwrap();},
        2 => { file_name = args.next().unwrap();
            start = args.next().unwrap().parse().unwrap(); },
        3 => { file_name = args.next().unwrap();
            start = args.next().unwrap().parse().unwrap();
            amount = args.next().unwrap().parse().unwrap(); },
        4 => {
            file_name = args.next().unwrap();
            start = args.next().unwrap().parse().unwrap();
            amount = args.next().unwrap().parse().unwrap();
            let temp_method:&str = &String::from(args.next().unwrap().trim());
            //let temp_method = args.next().unwrap().trim(); // <---- 다시보기 의문
            match temp_method {
                "w" => { method = MemoShowMethod::WORD; },
                "wm" => { method = MemoShowMethod::WORD_MEAN; },
                "wme" => { method = MemoShowMethod::WORD_MEAN_EXAMPLE; },
                &_ => { println!("incorrect show method !"); },
            }
        },
        _ => {println!(" ex)file_name start amount show_method:w,wm,wme!"); return;},
    }

    for argument in args {
        println!("{argument}");
    }

    let book = make_book(&file_name);
    let memo_manager = MemoManager::build(book);

    println!("시작부터 : 볼 갯수");
    memo_manager.display_memo(MemoShowRange::Select(start,amount),method);
    //memo_manager.display_memo(MemoShowRange::All);
}
