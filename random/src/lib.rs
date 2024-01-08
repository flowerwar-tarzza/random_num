use std::{fs,thread};
use std::time::Duration;
use std::io::{Write,stdin,stdout,Read};
use termion::{clear,cursor,async_stdin};
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

    // word manager
    // indexing, start to end
    // display method ,,각필드의 보여주기 선택
    //#[derive(Debug)]
    pub struct MemoManager{
        book : Vec<Memo>,
        total_memo: usize,
        i_start: usize,
        i_end: usize,
        i_current: usize,
        switch_word:bool,
        switch_mean:bool,
        switch_example:bool,
    }
    impl MemoManager {
        pub fn build(book:Vec<Memo>) -> MemoManager{
            let temp = MemoManager {
                total_memo: book.len(),
                i_start: 0,
                i_current: 0,
                i_end: book.len() - 1,
                book,
                switch_word:true,
                switch_mean:true,
                switch_example:true,
            };
            temp
        }

        pub fn display_memo_manual(&mut self) {
            self.i_current = self.i_start;
            let bottom_message = "[N]Next,[p]previous,[w][m][e]toggle,Range[r][q]Quit\n\r";
            let mut is_range_page:bool = false; // range key input :stdin borrow error
                                               // for c in stdin.keys() 내에 사용불가
                                               // :우회방법 으로 outer loop 사용

            let mut stdout = stdout().into_raw_mode().unwrap();
            write!(stdout,"{}\n\r","press any to start!").unwrap();
            stdout.flush().unwrap();
            'outter: loop {
                let mut stdin = stdin();
                let mut first_enter:bool = true; // 첫 출력시 self.i_current 증가 방지
                if is_range_page {  // 범위 지정을 위한 화면과 라인 읽기
                    write!(stdout,"{}{}",clear::All,cursor::Goto(1,1)).unwrap();
                    write!(stdout,"{}{}\n\r",
                           format!("range set({},{})",0,self.total_memo - 1),
                           cursor::Show).unwrap();
                    stdout.flush().unwrap();
                    let _ = stdout.suspend_raw_mode(); // stdout : show  key input
                    let input = TermRead::read_line(&mut stdin).unwrap().unwrap();
                    let _ = stdout.activate_raw_mode();
                    write!(stdout,"your input :{}\n\r",input).unwrap();
                    stdout.flush().unwrap();
                    self.set_indexs(input);
                    is_range_page = false;
                }
                // ==== key input control
                for c in stdin.keys() {
                    match c.unwrap() {
                        Key::Char('n') => {
                            if self.i_current < self.i_end && !first_enter {
                                self.i_current += 1;
                            }else {
                                first_enter = false;
                            }
                        },
                        Key::Char('p') => {
                            if self.i_current > self.i_start {
                                self.i_current -= 1
                            }
                        },
                        Key::Char('q') => {
                            write!(stdout,"{}{}",clear::All,cursor::Goto(1,1)).unwrap();
                            //stdout.flush().unwrap();
                            break 'outter
                        },
                        Key::Char('w') => self.switch_word = !self.switch_word,
                        Key::Char('m') => self.switch_mean = !self.switch_mean,
                        Key::Char('e') => self.switch_example = !self.switch_example,
                        Key::Char('a') => self.display_memo_auto_mode() ,
                        Key::Char('r') => {
                            is_range_page = true;
                            break;
                        },
                        _ => {
                            write!(stdout,"your input : other key\n\r").unwrap();
                            stdout.flush().unwrap();
                            continue;
                        },
                    }

                    //make output ----
                    let mut output = String::new();
                    let memo = &self.book[self.i_current];
                    if self.switch_word{
                        output_word(&mut output,&memo);
                    }
                    if self.switch_mean{
                        output_means(&mut output,&memo);
                    }
                    if self.switch_example{
                        output_examples(&mut output,&memo);
                    }
                    self.edit_output_for_test(&mut output);
                    write!(stdout,"{}{}",clear::All,cursor::Goto(1,1)).unwrap();
                    write!(stdout,"{}\n\r",output).unwrap();
                    write!(stdout,"[{}]\n\r",self.i_current).unwrap();
                    write!(stdout,"{}range({},{})",
                        bottom_message,self.i_start,self.i_end).unwrap();
                    stdout.flush().unwrap();
                }

            }
        }
        fn display_memo_auto_mode(&mut self) {
            let mut in_buff = async_stdin().bytes();
            let mut stdout = stdout().lock().into_raw_mode().unwrap();

            write!(stdout,"{}{}",clear::All,cursor::Goto(1,1)).unwrap();
            stdout.flush().unwrap();

            let buttom_message = "Exit Auto Mode : [q] / toggle switch : [w,m,e]/ replay :[r] ";
            loop {
                // key input check
                //let read = in_buff.next();
                //if let Some(Ok(b'q')) = read {
                    //break;
                //}

                let read = in_buff.next();
                let mut key = b'_';
                match read {
                    Some(val) => {key = val.unwrap();},
                    None => {},
                }
                //write!(stdout,"{}\n\r",read).unwrap();
                //stdout.flush().unwrap();
                
                match key {
                    b'q' => break,
                    b'w' => { self.switch_word = !self.switch_word},
                    b'm' => { self.switch_mean= !self.switch_mean},
                    b'e' => { self.switch_example = !self.switch_example},
                    b'r' => { self.i_current = self.i_start },
                    _ => {},
                }
                // make output
                let mut output = String::new();
                let memo = &self.book[self.i_current];
                if self.switch_word{
                    output_word(&mut output,&memo);
                }
                if self.switch_mean{
                    output_means(&mut output,&memo);
                }
                if self.switch_example{
                    output_examples(&mut output,&memo);
                }
                thread::sleep(Duration::from_millis(1000));
                write!(stdout,"{}{}\n\r",clear::All,cursor::Goto(1,1)).unwrap();
                write!(stdout,"{}\n\r",output).unwrap();
                write!(stdout,"{}\n\r",buttom_message).unwrap();
                stdout.flush().unwrap();

                // index for next word 
                if self.i_current < self.i_end { self.i_current += 1; }
                else {
                    write!(stdout,"reach end \n\r").unwrap();
                }
            }
        }
        fn set_indexs(&mut self,input:String) { // set index range (start, end)
            let v_inputs:Vec<_> = input.trim().split(',').collect();
            self.i_start = v_inputs[0].parse::<usize>().unwrap();
            self.i_end = v_inputs[1].parse::<usize>().unwrap();
            self.i_current = self.i_start;
            //validate input to iszie
            if self.i_end > self.total_memo - 1 {
                self.i_end %= self.total_memo;
            }
        }
        fn edit_output_for_test(&mut self,output:&mut String) {
            // 시험모드 : 사용자 입력 대기, word / mean 가림
            // example 에서 word 가림,
            let memo = &self.book[self.i_current];
            while output.contains(&memo.word) {
                let result= output.find(&memo.word);
                let start = match result {
                    Some(val) => val,
                    None => return,
                };
                let end = memo.word.len() + start;
                println!("start: {},end :{}",start,end);
                output.replace_range(start..end ,"<?>");
            }
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
    pub fn make_book(path:String) -> Vec<Memo> {
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
