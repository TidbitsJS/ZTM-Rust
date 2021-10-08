// Topic: Flow control using if..else

/**
    Program Requirements:
    * Displays a message based on the value of a boolean variable
    * When the variable is set to true, display "hello"
    * When the varaibale is set to false, display "goodbye"

    Notes:
    * Use a variable to set to either true or false
    * Use an if..else block to determine which message to display
    * Use the println macro to display messages to the terminal
**/

fn main() {
    let is_message: bool = true;

    if is_message {
        println!("hello");
    } else {
        println!("goodbye")
    }
}