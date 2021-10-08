// Topic: Working with expressions

/**
    Program requirements:
    * Print "its big" if a variable is > 100
    * Print "its small" if a variable is <= 100

    Notes:
    * Use a boolean variable set to the result of
        an if..else expression to store whether the value
        is > 100 or <= 100
    * Use a function to print the messages
    * Use a match expression to determine which message to print
**/

// display a message
fn display_message(is_greater : bool) {
    match is_greater {
        true => println!("its big"),
        false => println!("its small"),
    }
}

fn main() {
    let value = -110;
    let is_greater =  value >= 100;

    display_message(is_greater);
}
