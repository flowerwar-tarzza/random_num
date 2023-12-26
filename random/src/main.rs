use std::fs;
#[derive(Debug)]
struct Memo{
    word:String,
    pornounce:String,
    meanings:Vec<String>,
    ex_sentence:Vec<String>,
}

impl Memo {
    fn build() -> Memo {
        Memo{
            word : String::new(),
            pornounce: String::new(),
            meanings : Vec::new(),
            ex_sentence: Vec::new(),
            }
    }

}
fn make_book(path:&str) -> Vec<Memo> {
    let read_file = match fs::read_to_string(path) {
        Ok(result) => result,
        Err(e) => panic!("{e}"),
    };

    let mut lines:Vec<&str> = read_file.split('\n').collect();
    let mut memo_book : Vec<Memo> = Vec::new();
    lines.pop(); // 마지막 \n 의 "" 남김 제거


    for (i,line) in lines.iter().enumerate() {
        if i < 1 { continue; } // 칼럼 명 라인 생략

        let mut one_memo = Memo::build();
        let mut cols:Vec<_> = line.split('\t').collect();
        one_memo.word = String::from(cols.remove(0));
        one_memo.pornounce = String::from(cols.remove(0));

        let mut flag_ex_sentence: bool = false;
        for col in cols {
            //println!("col size:{}//{}",col,col.len());
            if col.len() > 0 && !flag_ex_sentence { // meaning add
                one_memo.meanings.push(String::from(col));
            } else {
                flag_ex_sentence = true;
            }
            if col.len() > 0 && flag_ex_sentence { // ex_sentence add
                one_memo.ex_sentence.push(String::from(col));
            }
        }
        memo_book.push(one_memo)
    }
    println!("const version test:{}",VERSION);
    memo_book

}
const VERSION:usize = 1111;
fn main() {
    let book = make_book("look_again_test.csv");
    //memo_book display
    println!("{:#?}",book);

}
