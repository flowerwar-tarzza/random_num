use std::fs;
use std::{thread,time,process};

pub mod memo {
    use super::*;

    #[derive(Debug)]
    pub struct Memo{
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

    // word manager
    // indexing, start to end
    // display method ,,각필드의 보여주기 선택
    //#[derive(Debug)]
    pub struct MemoManager{
        book : Vec<Memo>,
        total_memo: usize,
    }
    pub enum MemoDisplayMethod {
        All,
        Select(usize,usize),// start, amount
        OnlyOne(usize),
    }
    impl MemoManager {
        pub fn build(book:Vec<Memo>) -> MemoManager{
            MemoManager {
                total_memo: book.len(),
                book,
            }
        }

        pub fn display_memo(&self,method:MemoDisplayMethod) {
            let mut start:usize = 0;
            let mut end:usize = 0;
            let mut delay_time = time::Duration::from_millis(2000);

            match method {
                MemoDisplayMethod::All => { start = 0; end = self.total_memo; },
                MemoDisplayMethod::Select(head,amount) => {
                    start = head - 1;
                    if self.total_memo < amount {
                        end = start + amount - self.total_memo;
                    }else {
                        end = start + amount;
                    }
                },
                _ => { println!("not implemented!") },
            }

            for i in start..end {
                //print!("{esc}c",esc= 27 as char);
                process::Command::new("clear").status().unwrap();

                println!("{:#?}",self.book[i]);
                println!("[{}/{}] ",i + 1,end - start);

                if i == end - 1 {
                    continue;
                }
                thread::sleep(delay_time);
            }

        }
    }
    pub fn make_book(path:&str) -> Vec<Memo> {
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
        memo_book
    }
}

