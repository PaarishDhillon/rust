fn main(){
    let mut v  = Vec::new();
    v.push(7);
    v.push(3);
    v.push(1);
    v.push(12);
    v.push(4);
    v.push(8);
    println!("{:?}",v);
    
    let third = &v[2];
    println!("The third element is: {:?}",third);  // error not handled

    let fourth: Option<&i32> = v.get(3);
    match fourth {
        Some(fourth) => println!("The fourth element is: {fourth}"),
        None => println!("There is no fourth element"),  // error handling which was not done for third element
    }
    
    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
    match does_not_exist {
        Some(does_not_exist) => println!("The fourth element is: {does_not_exist}"),
        None => println!("There is no such element"),  // error handling which was not done for third element
    }
}