enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("May you day be nice");
            1
        },
        Coin::Nickel =>5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main(){
    let coin1: crate::Coin = Coin::Penny;
    let value = value_in_cents(coin1);
    println!("The value of a penny is {} cents", value);
}