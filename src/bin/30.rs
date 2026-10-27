enum ipAddkind{
    V4,
    V6
}

struct ipadd{
    kind: ipAddkind,
    address: String,
}

fn main(){
    let four = ipAddkind::V4;
    let six =  ipAddkind::V6;
    let home =  ipadd{
        kind: ipAddkind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = ipadd{
        kind: ipAddkind::V6,
        address: String::from("::1"),
    };
    println!("office ip address: {}",loopback.address);
    println!("home ip address: {}",home.address);
}