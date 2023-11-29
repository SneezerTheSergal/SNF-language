mod formatter;


use std::env::*;
use std::fs::read_to_string;

fn main() {
    let input: Vec<String> = args().collect();
    let file = input[1].clone();
    let contents = read_to_string(file.clone()).expect("expected a file");

    let mut fmt = formatter::Formatter::create(contents);
    fmt.format();

}
