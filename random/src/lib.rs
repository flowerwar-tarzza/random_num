use std::fs;
use std::{thread,time};
use std::io::{Write,stdin,stdout};
use termion::{clear,cursor};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;


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
    pub enum MemoShowMethod {
        Word,
        WordMean,
        WordMeanExample,
    }
    pub enum MemoShowRange {
        All,
        Select(usize,usize),// start, amount
    }

    // word manager
    // indexing, start to end
    // display method ,,각필드의 보여주기 선택
    //#[derive(Debug)]
    pub struct MemoManager{
        book : Vec<Memo>,
        total_memo: usize,
        i_start: usize,
        i_end: usize,
        memo_show_method: MemoShowMethod,
        switch_word:bool,
        switch_mean:bool,
        switch_example:bool,
    }
    impl MemoManager {
        pub fn build(book:Vec<Memo>,method:MemoShowMethod) -> MemoManager{
            let temp = MemoManager {
                total_memo: book.len(),
                i_start: 0,
                i_end: book.len() - 1,
                book,
                memo_show_method:method,
                switch_word:true,
                switch_mean:true,
                switch_example:true,
            };
            temp
        }

        pub fn display_memo_key_control(&mut self) {
            let mut cur_idx: usize = 0;
            let bottom_message = "[N]Next,[p]previous,[w][m][e]toggle,Range[r][q]Quit\n\r";
            let mut is_set_range:bool = false; // range key input :stdin borrow error
                                               // for c in stdin.keys() 내에 사용불가
                                               // :우회방법 으로 outer loop 사용

            let mut stdout = stdout().into_raw_mode().unwrap();
            write!(stdout,"{}\n\r","press any to start!"); stdout.flush().unwrap();
            'outter: loop {
                let mut stdin = stdin();
                let mut first_enter:bool = true; // 첫 출력시 cur_idx 증가 방지
                if is_set_range {
                    write!(stdout,"{}",clear::All); stdout.flush().unwrap();
                    write!(stdout,"{}{}\n\r","range set(start,end)",cursor::Show);
                    stdout.flush().unwrap();
                    stdout.suspend_raw_mode();
                    let mut input = TermRead::read_line(&mut stdin).unwrap().unwrap();
                    stdout.activate_raw_mode();
                    write!(stdout,"your input :{}\n\r",input); stdout.flush().unwrap();

                }
                // ==== key input
                for c in stdin.keys() {
                    match c.unwrap() {
                        Key::Char('n') => {
                            if cur_idx < self.total_memo - 1 && !first_enter
                            {cur_idx += 1}
                            else {first_enter = false;}
                        },
                        Key::Char('p') => { if cur_idx > 0 {cur_idx -= 1}},
                        Key::Char('q') => {
                            write!(stdout,"{}",clear::All); stdout.flush().unwrap();
                            break 'outter
                        },
                        Key::Char('w') => self.switch_word = !self.switch_word,
                        Key::Char('m') => self.switch_mean = !self.switch_mean,
                        Key::Char('e') => self.switch_example = !self.switch_example,
                        Key::Char('r') => {
                            is_set_range = true;
                            break;
                        },
                        _ => {
                            write!(stdout,"your input : other key\n\r");
                            stdout.flush().unwrap();
                            continue;
                        },
                    }

                    //make output ----
                    let mut output = String::new();
                    let memo = &self.book[cur_idx];
                    if self.switch_word{
                        output_word(&mut output,&memo);
                    }
                    if self.switch_mean{
                        output_means(&mut output,&memo);
                    }
                    if self.switch_example{
                        output_examples(&mut output,&memo);
                    }
                    write!(stdout,"{}{}",clear::All,cursor::Goto(1,1)); stdout.flush().unwrap();
                    write!(stdout,"{}\n\r",output);
                    write!(stdout,"[{}]\n\r",cur_idx);
                    write!(stdout,"{}",bottom_message);
                    stdout.flush().unwrap();
                }

            }
        }
        fn set_indexs(&self) {
            println!("not implements");
        }
    }
    fn output_word(output:&mut String,memo:&Memo){
        output.push_str(&format!("{} [{}]\n\r",memo.word,memo.pornounce));
    }
    fn output_means(output:&mut String,memo:&Memo) {
        for e in &memo.meanings {
            output.push_str(&format!("{}\n\r",e))
        }
    }
    fn output_examples(output:&mut String,memo:&Memo) {
        for e in &memo.ex_sentence{
            output.push_str(&format!("{}\n\r",e))
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn input_test() {
        println!("input_test in tests module ");
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).expect("read buffer error!");

    }
}
