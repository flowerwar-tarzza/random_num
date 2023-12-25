use std::fs;
#[derive(Debug)]
struct Memo{
    word:String,
    pornounce:String,
    meanings:Vec<String>,
    ex_sentence:Vec<String>,
}

impl Memo {
    //fn build(word:String,pornounce:String,meanings:Vec<String>,ex_sentence:Vec<String>) -> Memo {
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
    let read_file = match fs::read_to_string("look_again_test.csv") {
        Ok(result) => result,
        Err(e) => panic!("{e}"),
    };

    let lines:Vec<&str> = read_file.split('\n').collect();

    for (i,line) in lines.iter().enumerate() {
        if i < 1 {
            continue;
        }
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
        println!("{:#?}]",one_memo);
        if i > 3 {break;}  // 4 개까지 만  출력위해
    }

}
