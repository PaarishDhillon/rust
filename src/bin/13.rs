use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter the index");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: u32 = input.trim().parse().expect("Enter a number");
    let mut a = 0;
    let mut b = 1;
    let mut index = 0;
    while index < input {
        println!("{a}");
        let next = a + b;
        a = b;
        b = next;
        index += 1;
    }
}