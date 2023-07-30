use std::env;
use std::fs;

use nom::{bytes::complete::take_until, combinator::map, IResult};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Note {
    ByteString(Vec<u8>),
}

pub fn parse_head(contents: &[u8]) -> IResult<&[u8], &[u8]> {
    take_until("@")(contents)
}

pub fn parse(contents: &[u8]) -> IResult<&[u8], String> {
    map(parse_head, |bytes: &[u8]| {
        String::from_utf8(bytes.to_vec()).unwrap()
    })(contents)
}

pub fn load(file_path: &str) {
    let contents = fs::read(file_path).expect("Should have been able to read the file");
    let len = contents.len();

    println!("Read file with length {len}");
    let result = parse(&contents);
    println!("Result {result:?}")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);
    load(file_path);
}
