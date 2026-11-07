use std::result;

fn largest(list: &[i32]) -> &i32{
    let mut largest = &list[0];
    for item in list{
        if item > largest{
            largest = item;
        }
    }
    largest
}

fn main(){
    let list1 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result1 = largest(&list1);
    println!("The largest number is {result1}");
    // println!("The largest number is {}",largest(&list1));
    
    let list2 = vec![34, 50, 25, 100, 65];
    let result2 = largest(&list2);
    println!("The largest number is {result2}");
    // println!("The largest number is {}",largest(&list2));
}