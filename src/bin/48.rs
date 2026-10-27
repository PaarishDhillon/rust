use std::collections::HashMap;

fn main(){
    let text = "Hello I am Paarish Dhillon!, I am I am I am";
    let mut map = HashMap::new();
    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count+=1;
    }
    println!("{map:?}");
}