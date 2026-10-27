fn main(){
    let mut s1 = String::from("Hello"); //only one mutable reference can be given to a value
    let s2 = &mut s1;
    // let s3 = &mut s1; //This is a second mutable reference to s1 and hance will give an error 
    // println!("{s2} and {s3}");
    println!("{s2}");
}