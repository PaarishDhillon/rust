#[allow(unused)]
#[derive(Debug)]

struct Lang{
    language:String,
    version:String,
}
fn main(){
    let x = "Paarish";
    println!("Hello {}", x);
    println!("Hello {x}");
    let y = 4;
    println!("{0} X {0} = {1}",y, y*y);
    let bhasha = Lang {
        language: "Rust".to_string(),
        version: "1.823".to_string(),
    };
    println!("{:?}",bhasha);
    println!("{:#?}",bhasha);
}