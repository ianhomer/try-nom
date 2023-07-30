use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    fs::read(file_path)
        .expect("Should have been able to read the file");

    println!("Read file");
    //println!("With text:\n{contents}");
}
