struct Message{
    input: String,
}

impl Message {
    fn display(&self) -> String {
        self.input.clone()
    }
}

fn main(){
    let m1 = Message{
        input: String::from("Hello World"),
    };
    let m2 = (&m1).display();
    println!("{}",m2);
}