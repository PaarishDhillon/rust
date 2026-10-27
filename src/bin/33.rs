enum Option<T>{
    None,
    Some(T),
}

fn main(){
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // println!("{}",x+y); // This won't compile as option value is not a definite valid value
}   