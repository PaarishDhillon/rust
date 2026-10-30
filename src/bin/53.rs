use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error>{
    let mut username = String::new();
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/bin/random number.txt");
    File::open(path)?.read_to_string(&mut username)?;
    Ok(username)
}

fn main(){
    let mut s = read_username_from_file();
    println!("{s:?}");
}