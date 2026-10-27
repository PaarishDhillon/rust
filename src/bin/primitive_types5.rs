// struct animal{
//     name: String,
//     age: f32,
// }

// fn main() {
//     let cat = animal{
//         name: String::from("Furry McFurson"),
//         age: 3.5,
//     };

//     // TODO: Destructure the `cat` tuple in one statement so that the println works.
//     // let /* your pattern here */ = cat;

//     println!("{} is {} years old", cat.name, cat.age);
// }

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat;

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let /* your pattern here */ = cat;

    println!("{name} is {age} years old");
}