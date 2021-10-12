// Topic: Map combinator

/**
    Program requirements:
    * Given a user name, create and print out a user struct if the user exists.

    Note:
    * Use the existing find_user function to locate a user
    * Use the map function to create the user
    * Print out the User struct if found, or a "not found" message if not
**/

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String
}

/// Locates a user id based on the name.
fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}


fn main() {
    let user = find_user("sam").map(|id| User {
        user_id: id,
        name: "sam".to_owned(),
    });

    match user {
        Some(user) => println!("{:?}", user),
        None => println!("not found"),
    }
}
