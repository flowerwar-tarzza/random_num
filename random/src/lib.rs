use std::{thread,time::Duration};
use std::fs::{self,File};
use std::io::{self,Write,stdin,stdout,Read};
use termion::{clear,cursor,async_stdin};
use termion::{event::Key,input::TermRead,raw::IntoRawMode};
use std::collections::HashMap;
use chrono::{Local};

use rand::seq::SliceRandom;
use rand::thread_rng;

pub mod memo;


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
    #[test]
    fn conceal_example() {
        let word = "modest";
        let concealed_word = "?".repeat(word.len());
        let lower = "Modest people Do not hold their heads high, even when they remain at the top of their fields.";
        let mut indices = Vec::new();
        for (i,e) in lower.char_indices() {
            if e.is_uppercase() {indices.push(i);  }
        }

        let mut lower2 = lower.to_lowercase();
        let mut result = lower2.replace(word,&concealed_word);

        println!("word {}:------------>{}",word,concealed_word);
        println!("after replace:-lower2----------->{}",result);
        println!("-indices----------->{:?}",indices);

        //panic!("look at this");
        unsafe{
            let bytes = result.as_bytes_mut();
            for i in indices{
                if bytes[i] != b'?' {
                    bytes[i] = bytes[i] - (b'a'- b'A');
                }
            }
        };
        //println!("{}",std::str::from_utf8(example).unwrap());
        println!("after uppercase :-reult-------->{}",result);
    }

}
