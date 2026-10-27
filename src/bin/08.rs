fn main(){
    let mut count = 0;
    'counting: loop{
        println!("Count = {count}");
        let mut remaining = 10;
        loop{
            println!("remaining = {remaining}");
            if remaining == 9{
                break;
            }
            if count == 2{
                break  'counting;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}