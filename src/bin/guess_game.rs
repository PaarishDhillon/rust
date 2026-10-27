use rand::RngExt;
#[allow(unused)]
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut count = 1;
    println!("Guess the number!");
    let secret_number = rand::rng().random_range(1..=100);
    // println!("The secret number is: {secret_number}");
    // println!("Please input your guess.");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");
        // let guess:u32 = guess.trim().parse().expect("Please enter a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");
        println!("Please input your guess.");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Try higher"),
            Ordering::Greater => println!("Try lower"),
            Ordering::Equal => {
                println!("Congrats! You guessed it right");
                break;
            }
        }
        count = count + 1;
    }
    println!("You guessed the number in {count} guesses");
}
