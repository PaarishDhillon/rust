fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(6); // won't work as v is already borrowed immutably
    println!("The first element is: {first}");
   
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }
    
    println!("");

    v.push(178);
    
    for i in &v {
        println!("{i}");
    }
}