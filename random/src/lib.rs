use std::{fs,thread};
use std::time::Duration;
use std::io::{self,Write,stdin,stdout,Read};
use termion::{clear,cursor,async_stdin};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode};
use termion::cursor::DetectCursorPos;

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
    #[derive(PartialEq)]
    enum DisplayMode{
        TestWord, // word test: show Mean(conceiled word,example)
        TestMean, // mean test: show Word,example(conceiled mean)
        ShowAll,
    }
    enum Page{
        Main,
        Test,
        Learn,
        Auto,
        SetRange,
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
        display_mode:DisplayMode,
        page:Page,
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
                display_mode:DisplayMode::ShowAll,
                page:Page::Main,
            };
            temp
        }
        pub fn run(&mut self) {
            let mut stdout = stdout().into_raw_mode().unwrap();

            let book_info_string = format!("total memo: {}",self.total_memo);
            let control_message = format!("(L)earn:(T)est:(S)et Range:(A)uto:(Q)uit");

            'main: loop{
                let head_message = self.make_head_message();
                let stdin = stdin();
                write!(stdout,"{}{}",clear::All,cursor::Goto(1,1));
                write!(stdout,"{}\n\r",head_message);
                write!(stdout,"{}\n\r",book_info_string);
                write!(stdout,"{}\n\r",control_message);
                stdout.flush().unwrap();

                for c in stdin.keys() {
                    match c.unwrap() {
                        Key::Char('l') => { self.page = Page::Learn;
                            stdout.suspend_raw_mode();
                            self.page_learn();
                            stdout.activate_raw_mode();
                            break;} ,
                        Key::Char('t') => {  self.page = Page::Test;
                            stdout.suspend_raw_mode();
                            self.display_mode = DisplayMode::TestWord;
                            self.page_test();
                            stdout.activate_raw_mode();
                            break;} ,
                        Key::Char('a') => {  self.page = Page::Auto;
                            stdout.suspend_raw_mode();
                            self.page_auto();
                            stdout.activate_raw_mode();
                            break;} ,
                        Key::Char('s') => {  self.page = Page::SetRange;
                            stdout.suspend_raw_mode();
                            self.page_set_range();
                            stdout.activate_raw_mode();
                            break;},
                        Key::Char('q') => { break 'main;},
                        _ => {println!("other key pressed!\r");},
                    }
                }
            }
        }
        fn page_learn(&mut self) {
            self.i_current = self.i_start;
            let head_message = self.make_head_message();
            let bottom_message = "[N]Next,[p]previous,[w][m][e]toggle,[q]Quit\n\r";

            let mut stdout = io::stdout().into_raw_mode().unwrap();
            'outter: loop {
                let mut stdin = stdin();
               //make output ----
                let mut output = String::new();
                let memo = &self.book[self.i_current];
                if self.switch_word{
                    self.output_word(&mut output,&memo);
                }
                if self.switch_mean{
                    self.output_means(&mut output,&memo);
                }
                if self.switch_example{
                    self.output_examples(&mut output,&memo);
                }
                write!(stdout,"{}{}",clear::All,cursor::Goto(1,1)).unwrap();
                write!(stdout,"{}\n\r",head_message).unwrap();
                write!(stdout,"{}\n\r",output).unwrap();
                write!(stdout,"[{}]\n\r",self.i_current).unwrap();
                write!(stdout,"{}range({},{})",
                    bottom_message,self.i_start,self.i_end).unwrap();
                stdout.flush().unwrap();

                // ==== key input control
                for c in stdin.keys() {
                    match c.unwrap() {
                        Key::Char('n') => {
                            if self.i_current < self.i_end {
                                self.i_current += 1;
                            }
                        },
                        Key::Char('p') => {
                            if self.i_current > self.i_start {
                                self.i_current -= 1
                            }
                        },
                        Key::Char('q') => {
                            //write!(stdout,"{}{}",clear::All,cursor::Goto(1,1)).unwrap();
                            //stdout.flush().unwrap();
                            self.page = Page::Main;
                            break 'outter
                        },
                        Key::Char('w') => self.switch_word = !self.switch_word,
                        Key::Char('m') => self.switch_mean = !self.switch_mean,
                        Key::Char('e') => self.switch_example = !self.switch_example,
                        _ => {
                            write!(stdout,"your input : other key\n\r").unwrap();
                            stdout.flush().unwrap();
                            continue;
                        },
                    }
                    break;
                }

            }
        }
        fn page_set_range(&mut self) {
            let mut stdout =stdout().into_raw_mode().unwrap();
            write!(stdout,"{}{}",clear::All,cursor::Goto(1,1)).unwrap();
            write!(stdout,"{}{}\n\r",
                   format!("range set({},{})",0,self.total_memo - 1),
                   cursor::Show).unwrap();
            stdout.flush().unwrap();
            let _ = stdout.suspend_raw_mode(); // stdout : show  key input
            let input = TermRead::read_line(&mut stdin()).unwrap().unwrap();
            let _ = stdout.activate_raw_mode();
            write!(stdout,"your input :{}\n\r",input).unwrap();
            stdout.flush().unwrap();
            self.set_indexs(input);
        }
        fn page_auto(&mut self) {
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
                    self.output_word(&mut output,&memo);
                }
                if self.switch_mean{
                    self.output_means(&mut output,&memo);
                }
                if self.switch_example{
                    self.output_examples(&mut output,&memo);
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
        fn make_head_message(&self) ->String{
            let current_page = match self.page {
                Page::Main => {"Main Page"},
                Page::Test => {"Test page"},
                Page::Learn => {"Learn page"},
                Page::SetRange => {"Learn page"},
                Page::Auto => {"Auto Next page"},
            };
            format!("{}:({}:{})",current_page,self.i_start,self.i_end)
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
        fn page_test(&mut self) {
            let mut head_message = self.make_head_message();
            let mut stdout = stdout().into_raw_mode().unwrap();
            let bottom_message = "[q]Quit,[r]:retry,[a]:answer,\n\r\
                                  [n]:next,[p]:previous,[w]:WordMode,[m]:MeanMode\n\r";
            let mut can_input = false;


            //key event
            'outter: loop {
                let mut stdin = stdin();
                if can_input {
                    can_input = false;
                    write!(stdout,"\n\ranwser me:").unwrap();
                    stdout.flush().unwrap();

                    let _ = stdout.suspend_raw_mode();
                    let input_string = TermRead::read_line(&mut stdin).unwrap().unwrap();
                    let _ = stdout.activate_raw_mode();
                    if input_string.contains(&self.book[self.i_current].word) {
                        head_message.push_str(":0!");
                        if self.i_current < self.i_end {self.i_current += 1;}
                    }else {
                        head_message.push_str(":x!");
                    }
                }
                //make output ----
                let mut output = String::new();
                let memo = &self.book[self.i_current];
                if self.switch_word{
                    self.output_word(&mut output,&memo);
                }
                if self.switch_mean{
                    self.output_means(&mut output,&memo);
                }
                if self.switch_example{
                    self.output_examples(&mut output,&memo);
                }
                write!(stdout,"{}{}",clear::All,cursor::Goto(1,1)).unwrap();
                stdout.flush().unwrap();
                write!(stdout,"{}\n\r",head_message).unwrap();
                write!(stdout,"{}\n\r",output).unwrap();
                write!(stdout,"[{}]\n\r{}\n\r",self.i_current,bottom_message).unwrap();
                write!(stdout,"press enter to answer!:").unwrap();
                stdout.flush().unwrap();

                for c in stdin.keys() {
                    match c.unwrap() {
                        Key::Char('q') => {
                            self.display_mode = DisplayMode::ShowAll;
                            break 'outter;
                        },
                        Key::Char('n') => {
                            if self.i_current < self.i_end {self.i_current += 1;}
                        },
                        Key::Esc => {
                            can_input = true;
                        },
                        Key::Char('p') => {
                            if self.i_start < self.i_current {self.i_current -= 1;}
                        },
                        Key::Char('m') => {
                            if self.display_mode != DisplayMode::TestMean {
                                self.display_mode = DisplayMode::TestMean;
                            }
                        },
                        Key::Char('w') => {
                            if self.display_mode != DisplayMode::TestWord{
                                self.display_mode = DisplayMode::TestWord;
                            }
                        },
                        _ => {continue;},
                    }
                    break;
                }
            }
        }
       fn output_word(&self,output:&mut String,memo:&Memo){
            // test mode , display_mode,
            // make output
            match self.display_mode {
                DisplayMode::TestWord => {
                    let mut concealed_word = String::new();
                    for _i in memo.word.chars() {
                        concealed_word.push('_');
                    }
                    output.push_str(&format!("{}\n\r",concealed_word));
                },
                DisplayMode::TestMean => {
                    output.push_str(&format!("{} \n\r",memo.word));
                },
                DisplayMode::ShowAll => {
                    output.push_str(&format!("{} [{}]\n\r",memo.word,memo.pornounce));
                },
            }
        }
        fn output_means(&self,output:&mut String,memo:&Memo) {
            match self.display_mode {
                DisplayMode::TestWord => {
                    for e in &memo.meanings {
                        output.push_str(&format!("{}\n\r",e));
                    }
                },
                DisplayMode::TestMean => {
                    for _e in &memo.meanings {
                        output.push_str(&format!("?_____\n\r"));
                    }
                },
                DisplayMode::ShowAll => {
                    for e in &memo.meanings {
                        output.push_str(&format!("{}\n\r",e));
                    }
                },
            }
        }
        fn output_examples(&self,output:&mut String,memo:&Memo) {
            let concealed_word = String::from("?".repeat(memo.word.len()));
            match self.display_mode {
                DisplayMode::TestWord => {
                    for e in &memo.ex_sentence{
                        let concealed_example = e.replace(&memo.word,&concealed_word);
                        output.push_str(&format!("{}\n\r",concealed_example));
                    }
                },
                DisplayMode::TestMean => {
                    for e in &memo.ex_sentence{
                        output.push_str(&format!("{}\n\r",e))
                    }
                },
                DisplayMode::ShowAll => {
                    for e in &memo.ex_sentence{
                        output.push_str(&format!("{}\n\r",e))
                    }
                },
            }
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
        let s = "\n Hello\tworld\t\n";
        assert_eq!("Hello\tworld",s.trim());
        assert_eq!("\n Hello\tworld\t\n",s);
    }
}
