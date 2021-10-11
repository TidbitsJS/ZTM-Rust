fn main() {
    let numbers = vec![1, 2, 3];

    match numbers.is_empty() {
        true => println!("No numbers"),
        false => println!("Has numbers"),
    }
}
