use std::collections::{HashMap, btree_map::Entry::Occupied};

fn main(){
    let some = vec![1,1,1,1,1,1,2,23,3,4,5,64,32,22];
    let median: f64;
    let occur: usize;
    let mode: usize;
    let length = some.len();
    if length%2==0{
        let mid = (some[length/2]+some[(length/2)+1]) as f64;
        median = (mid/2.0) as f64;
    }
    else {
        median = some[(length/2)+1] as f64;
    }
    println!("The length of vector is: {length}");
    println!("The median is: {median}");
    let mut map = HashMap::new();
    for elements in &some{
        let count = map.entry(*elements).or_insert(0);
        *count+=1;
    }
    let mut key = Vec::with_capacity(map.len());
    let mut value = Vec::with_capacity(map.len());
    for (k,v) in &map{
        key.push(*k);
        value.push(*v);
    }
    println!("Key: {key:?}");
    println!("value: {value:?}");
    println!("map: {map:?}");
    value.sort();
    let value_length = value.len();
    let occur = value[value_length-1];
    if let Some((&mode_key,_)) = map.iter().find(|&(_,&v) | v == occur){
        mode = mode_key;
    }
    else {
        mode = 0;
    }
    println!("The mode is: {mode}");
    println!("The mode appears {occur} times");
}