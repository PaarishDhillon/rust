#[derive(Debug)]
enum SpreadSheetCell{
    Int(i32),
    Float(f32),
    Text(String),
}


fn main(){
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(2.8),
        SpreadSheetCell::Text(String::from("Hello World")),
    ];
    let something:Option<&SpreadSheetCell>  = row.get(2);
    match something{
        Some(something) => println!("{:?}",something),
        None => println!("There is nothing braw!"),
    }
}