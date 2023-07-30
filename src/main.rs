use std::env;
use std::fs;

use nom::IResult;

pub fn parse(file_path: &str) -> IResult<&str, &str> {
    let contents = fs::read(file_path).expect("Should have been able to read the file");
    let len = contents.len();

    println!("Read file with length {len}");
    Ok(("xx", ""))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);
    let result = parse(file_path);
    println!("Result {result:?}")
}
