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
fn main() {
    let mut read_file = match fs::read_to_string("look_again_test.csv") {
        Ok(result) => result,
        Err(e) => panic!("{e}"),
    };

    read_file.trim();
    let mut lines:Vec<&str> = read_file.split('\n').collect();
    lines.pop(); // 마지막 \n 의 "" 남김 제거
    let mut memo_book : Vec<Memo> = Vec::new();


    println!("lines num: {}",lines.len());
    for (i,line) in lines.iter().enumerate() {
        if i < 1 { continue; } // 칼럼 명 라인 생략
        println!("{},{}",i,line);
        let mut one_memo = Memo::build();
        let mut cols:Vec<_> = line.split('\t').collect();
        one_memo.word = String::from(cols.remove(0));
        one_memo.pornounce = String::from(cols.remove(0));
        println!("row length: {}",cols.len());
        let mut flag_ex_sentence: bool = false;
        for col in cols {
            //println!("col size:{}//{}",col,col.len());
            if col.len() > 0 && !flag_ex_sentence { // meaning add
                one_memo.meanings.push(String::from(col));
            } else {
                flag_ex_sentence = true;
            }
            if col.len() > 0 && flag_ex_sentence { // meaning add
                one_memo.ex_sentence.push(String::from(col));
            }
        }
        memo_book.push(one_memo);

        //println!("{:#?}]",one_memo);
        //if i > 9 {break;}  // 4 개까지 만  출력위해
    }
    //memo_book display
    println!("{:#?}",memo_book);

}
