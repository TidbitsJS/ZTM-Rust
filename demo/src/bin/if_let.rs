enum Color {
    Red,
    Blue,
    Green,
}


fn main() {
    let maybe_user = Some("Jerry");
    match maybe_user {
        Some(user) => println!("Hello, {:?}!", user),
        None => println!("No user found!"),
    }

    if let Some(user) = maybe_user {
        println!("Hello, {:?}!", user);
    } else {
        println!("No user found!");
    }

    let red = Color::Red;
    if let Color::Red = red {
        println!("It is red!");
    } else {
        println!("It is not red!");
    }
}
