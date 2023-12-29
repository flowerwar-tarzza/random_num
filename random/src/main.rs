use random::memo::*;
use std::env;

fn main() {
    let book = make_book("look_again_test.csv");
    let memo_manager = MemoManager::build(book);
    //memo_book display
    //println!("{:#?}",memo_manager);
    //memo_manager.display_memo(MemoDisplayMethod::All);
    println!("시작부터 : 볼 갯수");
    memo_manager.display_memo(MemoDisplayMethod::Select(1,4));
}
