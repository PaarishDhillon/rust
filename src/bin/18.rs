fn main(){
    let s1 = String::from("Hello, World");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}");
    // let s3 = s1; // This won't work since value of s1 has been transfeerred to the function
}

fn calculate_length(s:String) -> (String, usize) {
    let length = s.len();
    (s, length)
}