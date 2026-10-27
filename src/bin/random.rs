use rand::{RngExt, rng};

fn main(){
    let mut rng = rng();
    let n: u32 = rng.random();
    println!("Random numner = {}",n);
}