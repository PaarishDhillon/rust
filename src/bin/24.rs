fn main(){
    let mut s1 = String::from("Hello World");
    let word = first_word(&s1);
    println!("{word}");
    s1.clear();
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}