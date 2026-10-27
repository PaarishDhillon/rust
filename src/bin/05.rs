use std::io;

fn main(){
    let a = [1,2,3,4,5];
    for i in 0..5{
        println!("Element at index {i} is {}",a[i]);
    }

    let mut index = String::new();
    println!("Enter the index");

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read");
    let index:usize = index.trim().parse().expect("Input a number");
    let element = a[index];
    println!("The number in array at index {index} is {element}");
}