fn main(){
    let mut s1 = String::from("Hello"); //only one mutable reference can be given to a value
    change(&mut s1);
}

fn change(s: &mut String){
    s.push_str(", World");
    println!("{s}");
}