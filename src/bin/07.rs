fn main(){
    let mut count = 0;
    // loop{
    //     count+=1;
    //     println!("Hello World");
    //     if count == 10{
    //         break;
    //     }
    // }
    let mut result = loop{
        count+=1;
        println!("Hello World");
        if count == 10{
            break count*2;
        }
    };
    println!("The result is {:?}",result);
}