fn main(){
    let reference_to_nothing = dangle();
    println!("{reference_to_nothing}");
}

fn dangle() -> String{
    let s = String::from("Hello");
    s
}

// fn dangle() -> &String{
//     let s = String::from("Hello"); //This won't work as it returns a value which goes out of scope once this function ends
//     &s
// }