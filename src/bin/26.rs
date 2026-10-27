fn main(){
    let mut s1 = String::from("Hello World");
    let word = second_word(&s1);
    println!("The second word of string is: {word}");
    s1.clear();
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let length = s.len();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i+1..length];
        }
    }
    &s[..]
}