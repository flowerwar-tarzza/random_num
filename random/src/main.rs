use std::fs;

fn main() {
    let read_file = match fs::read_to_string("look_again_test.csv") {
        Ok(result) => result,
        Err(e) => panic!("{e}"),
    };
    println!("result {read_file}");

}
