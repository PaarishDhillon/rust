struct user{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> user{
    user{
    active: true,
    email,
    username,
    sign_in_count: 1,
    }
}

fn main(){
    let mut user1 = user{
        active: true,
        username: String::from("Paarish Dhillon"),
        email: String::from("paarishdhillon@gmail.com"),
        sign_in_count: 12,
    };
    // user1.email = String::from("dhillonpaarish@gmail.com");
    println!("Name: {}",user1.username);
    println!("Email id: {}",user1.email);
    let user2 = build_user(String::from("PD_BOI"), String::from("pdboi@gmail.com"));
    println!("name: {}, email: {}",user2.username, user2.email);

    // let mut user3  = user{
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("anotherexample@gmail.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    let mut user3 = user{
        email: String::from("someone@gmail.com"),
        ..user1
    };

    println!("\n");
    println!("user 3:- ");
    println!("Name: {}",user3.username);
    println!("Email: {}",user3.email);
    // println!("Name: {}",user1.username); // invalid as username is a string which was moved from user1 to user3
}