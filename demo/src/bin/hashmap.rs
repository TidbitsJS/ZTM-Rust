use std::collections::HashMap;

#[derive(Debug)]
struct Contents {
    content: String,
}

fn main() {
    let mut lockers = HashMap::new();
    lockers.insert(1, Contents { content: String::from("shirt") });
    lockers.insert(2, Contents { content: String::from("book") });
    lockers.insert(3, Contents { content: String::from("sweet") });

    for (locker_number, content) in lockers.iter() {
        println!("Locker number: {:?}, content: {:?}", locker_number, content.content);
    }
}
