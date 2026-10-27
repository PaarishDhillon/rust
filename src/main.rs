fn find_a(s:String)->Option<u32> 
{
    for (index, character) in s.chars().enumerate()
    {
        if character == 'a'
        {
            return Some(index as u32);
        }
    }
    return None;
}
fn main() {
    let str = String::from("Paarish");
    match find_a(str){
        Some(index) => println!("The letter a is found at index: {}",index),
        None => println!("The letter 'a' is not found in the string"),
    }
}