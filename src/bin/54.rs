use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error>{
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/bin/random number.txt");
    fs::read_to_string(path)
}

fn main(){
    let mut s = read_username_from_file();
    println!("{s:?}");
}