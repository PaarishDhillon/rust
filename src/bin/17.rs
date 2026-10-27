fn main(){
    let s = String::from("Hello, World");
    println!("{s}"); 
    takes_ownership(s); // ownership from s get transferred to the function, hence s is no longer valid
    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string:String){
    println!("{some_string}");
}

fn makes_copy(something:i32){
    println!("{something}");
}