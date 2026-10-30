use std::fs::File;
use std::io::ErrorKind;

fn main(){
    // let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/bin/random number.txt");
    // let greeting_file_result = File::open(path);
    // let greeting_file_result = File::open("random number.txt");
    let greeting_file_result = File::open("random numbers.txt");
    let greeting_file = match greeting_file_result{
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("random number.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Problem crreating the file: {e:?}"),
            },
            _=>{
                panic!("Problem opening the file: {error:?}");
            }
        }, 
    };
}