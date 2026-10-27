fn main(){
    let s1 = String::from("Hello, World");
    let length = calculate_length(&s1);
    println!("The length of '{s1}' is {length}.");
}

fn calculate_length(s: &String) -> usize{
    s.len()
}