// TODO: Fix the compiler error in the function without adding any new line.
fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    let vec: &mut Vec<i32> = vec;
    vec.push(88);

    vec.to_vec()
}

fn main() {
    // You can optionally experiment here.
    let v: &mut Vec<i32> = &mut vec![];
    fill_vec(v);
    println!("{v:?}");
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn move_semantics3() {
//         let vec0 = vec![22, 44, 66];
//         let vec1 = fill_vec(vec0);
//         assert_eq!(vec1, [22, 44, 66, 88]);
//     }
// }