use std::ffi::os_str::Display;

use crate::Message::input;

#[derive(Debug)]

enum Message{
    input(String),
}

impl Message {
    fn display(&self) -> String {
        match self{
            Message::input(text) => text.clone(),
        }
    }
}

fn main(){
    let m1:crate::Message = Message::input(String::from("Hello World"));
    // println!("{:?}",m1);
    let m2 = (&m1).display();
    println!("{:?}",m2);
}