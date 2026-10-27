use std::io;
use rand::RngExt;
use std::cmp::Ordering;

fn main(){
    let mut count = 0;
    let random_number = rand::rng().random_range(1..=100);
    println!("Guess the number");

    loop{
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 =  match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed {guess}");
        match guess.cmp(&random_number){
            Ordering::Less => println!("Try higher"),
            Ordering::Greater => println!("Try lower"),
            Ordering::Equal =>{
                println!("Congrats! You guessed it");
                break;
            }
        }
    println!("Please input your guess");
    count = count + 1;
    }
    println!("You guessed the number in {count} guesses");
}