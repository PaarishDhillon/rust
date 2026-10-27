fn main(){
    let data = "initial contents";
    let s = data.to_string();
    println!("{s}");
    let u = String::from("initial contents");
    println!("{u}");
    let mut s1 = String::from("Hello");
    let s2 = String::from(" World");
    s1.push_str(&s2);
    println!("{s1}");
    println!("{s2}");
    s1.push('w');
    println!("{s1}");
    let s3 = s1 + &s2;
    println!("{s3}");
}