fn main() {
    let tup = (500.0, 38, -1, "Paarish");
    let (a,b,c,d) = tup;
    println!("{}",a);
    let x = tup.0;
    println!("{}",x);
    println!("{}",tup.1);
}