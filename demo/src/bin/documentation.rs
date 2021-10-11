/// A favorite color.
enum Color {
    Red,
    Blue
}

/// A piece of mail.
struct Mail {
    /// The destination address.
    address: String,
}

/// Adds two numbers together.
fn add (a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum = add(5, 7);

    // Print sum to the console.
    println!("{:?}", sum);
}
