use std::fs::File;

fn main(){
    // let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/bin/random number.txt");
    // let greeting_file_result = File::open(path).unwrap();
    // let greeting_file_result = File::open("random number.txt").unwrap(); //Program panics
    let greeting_file_result = File::open("random number.txt")
        .expect("No such file exists"); 
    // let greeting_file = match greeting_file_result{
    //     Ok(file) => file,
    //     Err(error) => panic!("No such file detected: {error:?}"),
    // };
}