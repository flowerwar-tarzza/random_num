use std::{thread,time::Duration};
use std::fs::{self,File};
use std::io::{self,Write,stdin,stdout,Read};
use termion::{clear,cursor,cursor::DetectCursorPos,async_stdin};
use termion::{event::Key,input::TermRead,raw::IntoRawMode};
use std::collections::HashMap;
use chrono::{Local,DateTime};

use rand::seq::SliceRandom;
use rand::thread_rng;

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
    enum DisplayMode{ // refer this for makeing output string
        TestWordByMean,
        TestWordByExample,
        TestWordByBoth,
        TestMeanByWord,
        TestMeanByExample,
        TestMeanByBoth,
        ShowAll,
    }
    enum Page{
        Main,
        Test,
        Learn,
        Auto,
        SetRange,
        IncorrectWord,
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
        file_name:String,
    }
    impl MemoManager {
        pub fn build(book:Vec<Memo>,file_name:String) -> MemoManager{
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
                file_name,
            };
            temp
        }
        pub fn run(&mut self) {
            let mut stdout = stdout().into_raw_mode().unwrap();

            let book_info_string = format!("total memo: {}",self.total_memo);
            let control_message = format!("(L)earn:(T)est:(S)et Range:(I)ncorrect:(A)uto:(Q)uit");

            'main: loop{
                let head_message = self.make_head_message();
                let stdin = stdin();
                write!(stdout,"{}{}",clear::All,cursor::Goto(1,1)).unwrap();
                write!(stdout,"{}\n\r",head_message).unwrap();
                write!(stdout,"{}\n\r",book_info_string).unwrap();
                write!(stdout,"{}\n\r",control_message).unwrap();
                stdout.flush().unwrap();

                for c in stdin.keys() {
                    match c.unwrap() {
                        Key::Char('l') => { self.page = Page::Learn;
                            let _ = stdout.suspend_raw_mode();
                            self.page_learn();
                            let _ = stdout.activate_raw_mode();
                            break;} ,
                        Key::Char('t') => {  self.page = Page::Test;
                            let _ = stdout.suspend_raw_mode();
                            self.display_mode = DisplayMode::TestWordByBoth;
                            self.page_test();
                            let _ = stdout.activate_raw_mode();
                            break;} ,
                        Key::Char('a') => {  self.page = Page::Auto;
                            let _ = stdout.suspend_raw_mode();
                            self.page_auto();
                            let _ = stdout.activate_raw_mode();
                            break;} ,
                        Key::Char('s') => {  self.page = Page::SetRange;
                            let _ = stdout.suspend_raw_mode();
                            self.page_set_range();
                            let _ = stdout.activate_raw_mode();
                            break;},
                        Key::Char('i') => {  self.page = Page::IncorrectWord;
                            let _ = stdout.suspend_raw_mode();
                            self.page_incorrent_word();
                            let _ = stdout.activate_raw_mode();
                            break;
                        }
                        Key::Char('q') => { break 'main;},
                        _ => {println!("other key pressed!\r");},
                    }
                }
            }
        }
        fn page_incorrent_word(&self) {
            let mut stdout = stdout().into_raw_mode().unwrap();
            let head_message = self.make_head_message();
            let bottom_message = "q:quit,s:show detail";
            let read_file= fs::read_to_string("incorrect.log").unwrap();
            let logs:Vec<&str>= read_file.split('\n').collect();
            let mut contents = String::new();
            for line in &logs {
                contents.push_str(line);
                contents.push_str("\n\r");
            }

            //clear screen
            write!(stdout,"{}{}",clear::All,cursor::Goto(1,1)).unwrap();
            stdout.flush().unwrap();

            'outter: loop {
                let stdin = stdin();

                write!(stdout,"{}\n\r",head_message).unwrap();
                write!(stdout,"{}\n\r",contents).unwrap();
                write!(stdout,"{}\n\r",bottom_message).unwrap();
                stdout.flush().unwrap();
                for c in stdin.keys() {
                    match c.unwrap() {
                        Key::Char('q') => {
                            break 'outter;
                        },
                        Key::Char('s') => {
                        },
                        _ => {},
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
                let stdin = stdin();
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
                write!(stdout,"{}rakge({},{})",
                    bottom_message,self.i_start,self.i_end).unwrap();
                stdout.flush().unwrap();

                // ==== key input control
                for c in stdin.keys() {
                    match c.unwrap() {
                        Key::Char('n') | Key::Right => {
                            if self.i_current < self.i_end {
                                self.i_current += 1;
                            }
                        },
                        Key::Char('p') | Key::Left => {
                            if self.i_current > self.i_start {
                                self.i_current -= 1
                            }
                        },
                        Key::Char('q') => {
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

            write!(stdout,"{}{}",clear::All,cursor::Goto(1,1)).unwrap(); stdout.flush().unwrap();

            let buttom_message = "Exit Auto Mode : [q] / toggle switch : [w,m,e]/ replay :[r] ";
            loop {
                let read = in_buff.next();
                let mut key = b'_';
                match read {
                    Some(val) => {key = val.unwrap();},
                    None => {},
                }
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
                Page::IncorrectWord => "Incorrect Word page",
            };
            format!("{}:{}-({}:{})",current_page,self.file_name,self.i_start,self.i_end)
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

        fn switch_all_on(&mut self) {
            self.switch_word =true;
            self.switch_mean =true;
            self.switch_example =true;
        }
        fn page_test(&mut self) {
            self.switch_all_on();
            //let mut tr_correct:HashMap<usize,Vec<&str>> = HashMap::new();
            let mut tr_incorrect:HashMap<usize,Vec<String>> = HashMap::new();

            let head_message = self.make_head_message();
            let mut stdout = stdout().into_raw_mode().unwrap();
            let bottom_message = "[q]Quit,[r]:retry,[Enter]:answer,\n\r\
                                  [s]:switch test mode (word <-> mean),\
                                  [h]:hint switch,\
                                  [x]:show incorrect answer\n\r\
                                  ";
            let mut can_input = false;
            let mut reach_end = false;
            let mut next_word = false;

            //shuffle indexs
            let mut rng = thread_rng();
            let mut shuffled_indexes:Vec<_> = (self.i_start..= self.i_end).into_iter().collect();
            shuffled_indexes.shuffle(&mut rng);

            let mut test_result_message = String::new();

            self.i_current = shuffled_indexes.pop().unwrap();
            //key event , screen display
            'outter:
            loop {
                let mut stdin = stdin();

                if can_input && !reach_end {
                    can_input = false;
                    write!(stdout,"\n\ranwser me:").unwrap();
                    stdout.flush().unwrap();

                    let _ = stdout.suspend_raw_mode();
                    let mut input_string = TermRead::read_line(&mut stdin).unwrap().unwrap();
                    let _ = stdout.activate_raw_mode();
                    // check the answer for the memo
                    // inert the result in test result maps:tr_correct,tr_incorrect

                    if input_string.contains("next") { // 틀렸을때 다음으로..
                        next_word = true;
                        write_tr_incorrect(&mut tr_incorrect,&mut input_string,self.i_current);
                        test_result_message=":x!".to_string();
                    }else {
                        if input_string.contains(&self.book[self.i_current].word) {
                            test_result_message=":0!".to_string();
                            next_word = true;
                        }else {
                            write_tr_incorrect(&mut tr_incorrect,&mut input_string,self.i_current);
                            test_result_message=":x!".to_string();
                        }
                    }

                    // Next word
                    if next_word {
                        if shuffled_indexes.len() > 0 {
                            self.i_current = shuffled_indexes.pop().unwrap();
                        }else {
                            test_result_message.push_str("reach end!");
                            reach_end = true;
                            save_incorrect_log(&tr_incorrect,self.i_start,self.i_end);
                        }
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
                test_result_message.push_str(&format!("{}",shuffled_indexes.len()));
                write!(stdout,"{}{}",clear::All,cursor::Goto(1,1)).unwrap();
                stdout.flush().unwrap();
                write!(stdout,"{}\n\r",head_message).unwrap();
                write!(stdout,"{}",output).unwrap();
                write!(stdout,"[{}]{}\n\r{}",self.i_current,test_result_message,bottom_message).unwrap();
                write!(stdout,"press enter to answer!:").unwrap();
                stdout.flush().unwrap();

                for c in stdin.keys() {
                    match c.unwrap() {
                        Key::Char('q') => {
                            self.display_mode = DisplayMode::ShowAll;
                            break 'outter;
                        },
                        Key::Char('\n')=> {
                            can_input = true;
                        },
                        Key::Char('h') => {
                            if self.display_mode == DisplayMode::TestWordByMean{
                                self.display_mode = DisplayMode::TestWordByExample;
                            }
                            else if self.display_mode == DisplayMode::TestWordByExample{
                                self.display_mode = DisplayMode::TestWordByMean;
                            }
                            else if self.display_mode == DisplayMode::TestMeanByExample{
                                self.display_mode = DisplayMode::TestMeanByWord;
                            }
                            else if self.display_mode == DisplayMode::TestMeanByWord{
                                self.display_mode = DisplayMode::TestMeanByExample;
                            }
                            else if self.display_mode == DisplayMode::TestWordByBoth{
                                self.display_mode = DisplayMode::TestWordByMean;
                            }
                            else if self.display_mode == DisplayMode::TestMeanByBoth{
                                self.display_mode = DisplayMode::TestMeanByWord;
                            }
                        },
                        Key::Char('s') => {
                            match self.display_mode {
                                DisplayMode::TestWordByBoth | DisplayMode::TestWordByMean | DisplayMode::TestWordByExample
                                => self.display_mode = DisplayMode::TestMeanByBoth ,
                                DisplayMode::TestMeanByBoth | DisplayMode::TestMeanByWord | DisplayMode::TestMeanByExample
                                => self.display_mode = DisplayMode::TestWordByBoth,
                                _ => {},
                            }
                        },
                        Key::Char('x') => {
                            for (key,_value) in &tr_incorrect {
                                println!("{}\r",self.book[*key].word); // --- *key
                            }
                            thread::sleep(Duration::from_millis(2000));

                        }
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
                DisplayMode::TestWordByBoth | DisplayMode::TestWordByMean | DisplayMode::TestWordByExample => {
                    let mut concealed_word = String::new();
                    for _i in memo.word.chars() {
                        concealed_word.push('?');
                    }
                    output.push_str(&format!("{}\n\r",concealed_word));
                },
                DisplayMode::TestMeanByWord | DisplayMode::TestMeanByBoth => {
                    output.push_str(&format!("{} \n\r",memo.word));
                },
                DisplayMode::ShowAll => {
                    output.push_str(&format!("{} [{}]\n\r",memo.word,memo.pornounce));
                },
                _ => {},
            }
        }
        fn output_means(&self,output:&mut String,memo:&Memo) {
            match self.display_mode {
                DisplayMode::TestWordByBoth | DisplayMode::TestWordByMean => {
                    for e in &memo.meanings {
                        output.push_str(&format!("{}\n\r",e));
                    }
                },
                DisplayMode::TestMeanByBoth | DisplayMode::TestMeanByWord | DisplayMode::TestMeanByExample => {
                    for _e in &memo.meanings {
                        output.push_str(&format!("?_____\n\r"));
                    }
                },
                DisplayMode::ShowAll => {
                    for e in &memo.meanings {
                        output.push_str(&format!("{}\n\r",e));
                    }
                },
                _ => {},
            }
        }
        fn output_examples(&self,output:&mut String,memo:&Memo) {
            let concealed_word = String::from("?".repeat(memo.word.len()));
            match self.display_mode {
                DisplayMode::TestWordByBoth | DisplayMode::TestWordByExample => {
                    for e in &memo.ex_sentence{
                        let concealed_example = e.replace(&memo.word,&concealed_word);
                        output.push_str(&format!("{}\n\r",concealed_example));
                    }
                    let _ = output.trim();
                },
                DisplayMode::TestMeanByBoth | DisplayMode::TestMeanByExample => {
                    for e in &memo.ex_sentence{
                        output.push_str(&format!("{}\n\r",e))
                    }
                },
                DisplayMode::ShowAll => {
                    for e in &memo.ex_sentence{
                        output.push_str(&format!("{}\n\r",e))
                    }
                },
                _ => {},
            }
        }
    }
    fn save_incorrect_log(tr_incorrect:&HashMap<usize,Vec<String>> ,i_start:usize,i_end:usize) {
        let fd = File::options().create(true).append(true).open("incorrect.log");
        let mut date_time =format!("{}", Local::now().format("%Y/%m/%d %H:%M"));
        date_time.push(':');
        let mut contents_for_log = String::new();
        match fs::read_to_string("incorrect.log") {
            Ok(val) => {
                if !val.contains(&date_time) {
                    contents_for_log.push_str(&date_time);
                    contents_for_log.push_str(&format!("range({},{}) ",i_start,i_end));
                }
            },
            Err(_err) => {},
        }
        for (key,value) in tr_incorrect {
            let mut element = String::new();
            element.push('(');
            element.push_str(&key.to_string());
            element.push(',');
            element.push_str(&value.len().to_string());
            element.push('-');
            for e in value {
                element.push_str(e);
            }
            element.push_str(")");
            contents_for_log.push_str(&element);
        }
        contents_for_log.push('\n');
        write!(fd.expect("file read error"),"{}",contents_for_log).unwrap();
    }

    fn write_tr_incorrect(tr_incorrect:&mut HashMap<usize,Vec<String>>,input_string:&mut String,i_current:usize){
        input_string.push(',');//add delimiter
        tr_incorrect.entry(i_current).
        or_insert(Vec::new()).
        push(input_string.clone());
    }
    pub fn make_book(path:&String) -> Vec<Memo> {
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
    use std::fs::{self,File};
    use chrono::{Local,DateTime};

    #[test]
    fn input_test() {
        let s = "\n Hello\tworld\t\n";
        assert_eq!("Hello\tworld",s.trim());
        assert_eq!("\n Hello\tworld\t\n",s);

        let mut rng = thread_rng();
        //let mut deck = [1,2,3,4,5,6,7];
        let a = 1;
        let b = 50;
        let mut deck:Vec<_> = (a..b).into_iter().collect();
        println!("Unshuffled: {:?}",deck);
        deck.shuffle(&mut rng);
        println!("shuffled: {:?}",deck);
    }

    #[test]
    fn test_hashmap(){
        let mut tresult:HashMap<usize,Vec<&str>> = HashMap::new();
        let default = "ox";
        let mut tvec = Vec::new();
        *&tvec.push("test value");
        tresult.entry(3).or_insert(Vec::new()).push("x");
        tresult.entry(3).or_insert(Vec::new()).push("o");

        println!("{:?}",tresult);
        println!("{:?}",tvec);
    }
    #[test]
    fn test_fs(){
        let now = Local::now();
        let formatted = &format!("{}",now.format("%Y/%m/%d %H:%M"));
        let contents = match fs::read_to_string("test2.txt"){
            Ok(val) => val,
            Err(err) =>String::new(),
        };
        let mut write_string = String::new();
        if contents.contains(formatted) {
            write_string.push_str("파일에 쓸내용");
        }else {
            write_string.push_str(&format!("{}\n{}",formatted,"파일에 쓸내용"));
        }
        // file write test
        let mut fd = File::options().create(true).
            append(true).open("test2.txt").expect("file open error");
        match writeln!(fd,"{}",write_string) {
            Ok(val) => {},
            Err(err) => { println!( "{err}" ); },
        }
    }
}
