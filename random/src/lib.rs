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
        pronunciation:String,
        meanings:Vec<String>,
        ex_sentence:Vec<String>,
    }
    impl Memo {
        fn build() -> Memo {
            Memo{
                word : String::new(),
                pronunciation: String::new(),
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
        OpenLog,
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
        farthest_index:usize,
    }
    impl MemoManager {
        pub fn build(book:Vec<Memo>,file_name:String) -> MemoManager{
            let log_file = "data.log";
            let farthest_index = read_farthest_index(log_file);
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
                farthest_index,
            };
            temp
        }
        pub fn run(&mut self) {
            let mut stdout = stdout().into_raw_mode().unwrap();

            let book_info_string = format!("total memo: {},farthest_index: {}",
                                           self.total_memo,self.farthest_index);
            let control_message = format!("(S)et Range:(L)earn:(T)est:(O)pen log:(A)uto:(Q)uit");

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
                        Key::Char('o') => {  self.page = Page::OpenLog;
                            let _ = stdout.suspend_raw_mode();
                            self.page_log();
                            let _ = stdout.activate_raw_mode();
                            break;
                        }
                        Key::Char('q') => { break 'main;},
                        _ => {println!("other key pressed!\r");},
                    }
                }
            }
        }
        fn page_log(&self) {
            let file_path = "data.log";
            let mut stdout = stdout().into_raw_mode().unwrap();
            let mut need_details = false;
            let head_message = self.make_head_message();
            let bottom_message = "q:quit,s:show detail";

            let read_file = match fs::read_to_string(file_path){
                Ok(val) => val,
                Err(e) => return,
            };

            let logs:Vec<&str>= read_file.split('\n').collect();
            let mut contents = String::new();

            for (i,line) in logs.iter().enumerate() {
                let bk_line = line.splitn(4,' ').collect::<Vec<&str>>();
                //println!("bk_line--->{:?}\r",bk_line);
                if bk_line.len() > 3  {
                    contents.push_str(&format!("{:>4}",(i.to_string() + ") "))); //add index )

                    let mut wrongs =  "";
                    for (i,col) in bk_line.into_iter().enumerate() { //cols:date time range wrongs
                        if i == 3 {// 3 wrongs : escapte
                            wrongs = col.trim_start_matches("(").trim_end_matches(",)");
                            break;
                        }
                        contents.push_str(col);
                        contents.push(' ');
                    }
                    if wrongs == "" {
                        contents.push_str("\n\r");
                        continue;
                    }
                    // ------ wrong inputs handle ------------
                    let bk_wrongs = wrongs.split(")(").collect::<Vec<&str>>();
                    for sp_idx_inputs in bk_wrongs {
                        let mut sp_idx_input = sp_idx_inputs.split(',').collect::<Vec<&str>>();
                        //println!("sp_idx_input--->{:?}\r",sp_idx_input);
                        contents.push('(');
                        contents.push_str(&(sp_idx_input[0].to_string() + "-"));//index
                        contents.push_str(&sp_idx_input[1].to_string());//first wrong word
                        let _ = sp_idx_input.pop();
                        //println!("//sp_idx_input--->{:?}\r",sp_idx_input);
                        if sp_idx_input.len() >= 3 {
                            contents.push_str(",..."); //  ellipsis
                        }
                        contents.push(')');
                    }
                }
                contents.push_str("\n\r");
            }

            //clear screen
            write!(stdout,"{}{}",clear::All,cursor::Goto(1,1)).unwrap();
            stdout.flush().unwrap();


            'outter: loop {
                let mut stdin = stdin();

                write!(stdout,"{} {}",clear::All,cursor::Goto(1,1)).unwrap();
                write!(stdout,"{}\n\r",head_message).unwrap();
                write!(stdout,"{}\n\r",contents).unwrap();
                stdout.flush().unwrap();
                // ---- detail input ---
                if need_details {
                    write!(stdout,"input line_index for detail information:\n\r");
                    let _ = stdout.suspend_raw_mode().unwrap();
                    stdout.flush().unwrap();
                    let line_index = TermRead::read_line(&mut stdin).unwrap().unwrap();
                    let _ = stdout.activate_raw_mode().unwrap();
                    stdout.flush().unwrap();
                    println!("{}\r",line_index);
                    need_details = false;

                    //-----get index wrongs------ -
                    let mut index_wrongs:Vec<_> = Vec::new();
                    match read_log_indexed(&line_index,file_path){
                    //-> Result<Vec<(usize,Vec<String>)>,String>
                        Ok(val) => index_wrongs = val,
                        Err(e) => { println!("{}\n\r",e); },
                    };

                    //----display correct word / wrong answers -(usize,Vec<String>)---
                    if !index_wrongs.is_empty() {
                        write!(stdout,"{}{}",clear::All,cursor::Goto(1,1));
                        stdout.flush().unwrap();
                        for (index ,wrongs)in index_wrongs {
                            write!(stdout,"o>{} [{}]: ",self.book[index].word,
                                   self.book[index].pronunciation);
                            for mean in &self.book[index].meanings {
                                write!(stdout,":{}",mean);
                            }
                            write!(stdout,"\r\n");
                            for wrong in wrongs{
                                write!(stdout,"x>{}\n\r",wrong);
                            }
                            write!(stdout,"\n\r");
                        }
                        stdout.flush().unwrap();
                    }
                }
                write!(stdout,"{}\n\r",bottom_message).unwrap();
                stdout.flush().unwrap();
                for c in stdin.keys() {
                    match c.unwrap() {
                        Key::Char('q') => {
                            break 'outter;
                        },
                        Key::Char('s') => {
                            need_details = true;
                            break;
                        },
                        _ => {break;},
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
            let mut head_message = String::new();
            let mut body_message = String::new();
            let mut input = String::new();
            let mut set_input = false;
            let end_message = "q:quit,o:open log,s:set_range";
            'outter: loop {
                write!(stdout,"{}{}",clear::All,cursor::Goto(1,1)).unwrap();

                head_message = format!("Available range : {}-{}",0,self.total_memo - 1);

                write!(stdout,"{}\n\r",head_message).unwrap();
                write!(stdout,"{}\n\r",format!("farthest index({})",self.farthest_index)).unwrap();
                write!(stdout,"{}\n\r",format!("set index({},{})",self.i_start,self.i_end)).unwrap();
                if !set_input {write!(stdout,"{}\n\r",end_message).unwrap();}
                stdout.flush().unwrap();

                if set_input {
                    write!(stdout,"new range:").unwrap();
                    stdout.flush().unwrap();
                    let _ = stdout.suspend_raw_mode(); // stdout : show  key input
                    input = TermRead::read_line(&mut stdin()).unwrap().unwrap();
                    let _ = stdout.activate_raw_mode();
                    //write!(stdout,"your input :{}\n\r",input).unwrap();
                    self.set_indexs(input);
                    set_input= false;
                    continue;
                }
                for c in stdin().keys() {
                    match c.unwrap() {
                        Key::Char('q') => { break 'outter; },
                        Key::Char('s') => { set_input = true; break; },
                        _ => {break;},
                    }
                }
            }
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
        fn page_test(&mut self) {
            self.switch_all_on();
            let mut tr_incorrect:HashMap<usize,Vec<String>> = HashMap::new();

            let head_message = self.make_head_message();
            let mut stdout = stdout().into_raw_mode().unwrap();
            let bottom_message = "[q]Quit,[r]:retry,[Enter]:answer,\n\r\
                                  [s]:switch test mode (word <-> mean),\
                                  [h]:hint switch,\
                                  [x]:show incorrect answer\n\r\
                                  ";
            let mut can_answer = false;
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

                if can_answer && !reach_end {
                    can_answer = false;
                    write!(stdout,"\n\r\"Next\" for next word!\n\ranwser me:").unwrap();
                    stdout.flush().unwrap();

                    let _ = stdout.suspend_raw_mode();
                    let mut input_string = TermRead::read_line(&mut stdin).unwrap().unwrap();
                    let _ = stdout.activate_raw_mode();

                    // check the answer for the memo.word
                    // inert the result in test result maps:tr_correct,tr_incorrect
                    if input_string.contains("next") { // 틀렸을때 다음으로..
                        next_word = true;
                        write_test_result(&mut tr_incorrect,&mut input_string,self.i_current);
                        test_result_message=":x!".to_string();
                    }else {
                        if input_string.contains(&self.book[self.i_current].word) {
                            test_result_message=":0!".to_string();
                            next_word = true;
                        }else {
                            write_test_result(&mut tr_incorrect,&mut input_string,self.i_current);
                            next_word = false;
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
                            write_log(&tr_incorrect,self.i_start,self.i_end);
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
                if !reach_end {
                    write!(stdout,"press enter to answer!:").unwrap();
                }
                stdout.flush().unwrap();

                for c in stdin.keys() {
                    match c.unwrap() {
                        Key::Char('q') => {
                            self.display_mode = DisplayMode::ShowAll;
                            break 'outter;
                        },
                        Key::Char('\n')=> { // enter key
                            can_answer = true;
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

        fn make_head_message(&self) ->String{
            let current_page = match self.page {
                Page::Main => {"Main Page"},
                Page::Test => {"Test page"},
                Page::Learn => {"Learn page"},
                Page::SetRange => {"Learn page"},
                Page::Auto => {"Auto Next page"},
                Page::OpenLog => "Word Log page",
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
                    output.push_str(&format!("{} [{}]\n\r",memo.word,memo.pronunciation));
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
    pub fn read_log_indexed(line_index:&str,path:&str) -> Result<Vec<(usize,Vec<String>)>,String> {
        let log_all = match fs::read_to_string(path) {
            Ok(val) => val,
            Err(e) => return Err(e.to_string()),
        };
        let mut logs:Vec<&str> = log_all.split('\n').collect();
        let _ = logs.pop(); // eleminate last ""

        let index = match line_index.parse::<usize>(){
            Ok(val) => {
                if val >= logs.len() {
                    logs.len() - 1
                }else {
                    val
                }
            },
            Err(e) => return Err(format!("inde error: {}",e)),
        };
        let selected = logs[index];

        // Get incorrect words and indices
        let splited = selected.splitn(4,' ').collect::<Vec<_>>();

        //----리턴값 처리 ......
        // 날짜시간,범위,틀린단어,
        // splited vector 각요소 중 index, incorrect word 듀플
        // 벡터로 저장 리턴함.
        let incorrects = splited[3].split(")(").collect::<Vec<&str>>();
        if incorrects[0] == "" {
            return Err("It doesn't have wrong answer".to_string())
        }
        let mut result = Vec::new();
        for element in &incorrects {

            let mut one_idx_wrgs:(usize,Vec<String>) = (0,Vec::new());
            let trimed_ele= element.trim_start_matches('(').trim_end_matches(')');
            let mut index_wrongs:Vec<&str> = trimed_ele.split(',').collect();
            one_idx_wrgs.0 = index_wrongs.remove(0).parse::<usize>().unwrap();
            let _ = index_wrongs.pop();

            for wrong in index_wrongs {
                if wrong.contains("next") {break;}
                one_idx_wrgs.1.push(wrong.to_string());
            }
            result.push(one_idx_wrgs.clone());
        }

        //Ok(vec![(1,vec!["test".to_string(),"test2".to_string()])]) //dumy
        Ok(result)
    }
    pub fn read_farthest_index(file_path:&str) -> usize{
        let contents = fs::read_to_string(file_path).unwrap();

        //println!("{:}",contents);
        let mut v_lines = contents.split('\n').collect::<Vec<&str>>();
        let _ = v_lines.pop();
        //println!("v_lines--->{:#?}",v_lines);
        let mut farthest_index = 0;
        for line in v_lines {
            let range = line.split("range").collect::<Vec<&str>>()[1]
                .split(' ').collect::<Vec<&str>>()[0]
                .trim_start_matches("(").trim_end_matches(")")
                .split(',').collect::<Vec<&str>>();
            println!("range -->{:?}",range);
            let index = range[1].parse::<usize>().unwrap();
            if farthest_index < index {
                farthest_index = index;
            }
        }
        farthest_index
    }
    fn write_log(tr_incorrect:&HashMap<usize,Vec<String>> ,i_start:usize,i_end:usize) {
        let fd = File::options().create(true).append(true).open("data.log");
        let mut date_time =format!("{}", Local::now().format("%Y/%m/%d %H:%M"));
        let mut contents_for_log = String::new();
        match fs::read_to_string("data.log") {
            Ok(val) => {
                if !val.contains(&date_time) { //동시간 에 대한 처리
                    contents_for_log.push_str(&date_time);
                    contents_for_log.push_str(&format!(" range({},{}) ",i_start,i_end));
                }
            },
            Err(_err) => {}, // data.log 부재시 처리 불필요.  File::options().caeate속성
        }
        for (key,value) in tr_incorrect {
            let mut element = String::new();
            element.push('(');
            element.push_str(&key.to_string());
            element.push(',');
            for e in value {
                element.push_str(e);
            }
            element.push_str(")");
            contents_for_log.push_str(&element);
        }
        contents_for_log.push('\n');
        write!(fd.expect("file read error"),"{}",contents_for_log).unwrap();
    }
    fn write_test_result(tr_incorrect:&mut HashMap<usize,Vec<String>>,input_string:&mut String,i_current:usize){
        input_string.push(',');//add delimiter
        tr_incorrect.entry(i_current). //  current index as key
        or_insert(Vec::new()).    // &mut return
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
            one_memo.word = String::from(cols.remove(0).trim());//벡터 첫요소,공백문자 처리
            one_memo.pronunciation = String::from(cols.remove(0));

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
    use super::memo;
    use std::io;
    use std::fs::{self,File};
    use chrono::{Local,DateTime};
    use std::env::args;

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
    fn test_write_datetime(){
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
    #[test]
    fn read_log_indexed_for_search_info() {
        //find necessary info to return caller

        let line_index = args().nth(2).expect("no line_index");
        let path = args().nth(3).expect("no file path");
        println!("command line args::{:#?}",line_index);
        let result = memo::read_log_indexed(&line_index,&path);
        match result{
            Ok(val) => println!("{:#?}",val),
            Err(e) => println!("{}",e),
        }
    }
    #[test]
    fn read_farthest_index(){
        let mut args = args();
        let file_path = args.nth(2).unwrap();
        let index = memo::read_farthest_index(&file_path);
        println!("test: fn read_farthest====>{}",index);
    }
}
