use std::fs;

struct Memo{
    word:String,
    pornounce:String,
    meanings:Vec<String>,
    ex_sentence:Vec<String>,
}

impl Memo {
    fn build(word:String,pornounce:String,meanings:Vec<String>,ex_sentence:Vec<String>) -> Memo {
        Memo{
            word,
            pornounce,
            meanings,
            ex_sentence,
            }
    }
}
fn main() {
    let read_file = match fs::read_to_string("look_again_test.csv") {
        Ok(result) => result,
        Err(e) => panic!("{e}"),
    };

    let v:Vec<&str> = read_file.split('\n').collect();

    for (i,line) in v.iter().enumerate() {
        if i < 1 {
            continue;
        }
        let row:Vec<_> = line.split('\t').collect();
        
        for col in row {
            println!("{}",col);
        }
        //println!("{}=> {}]",i,line);
    }
}
