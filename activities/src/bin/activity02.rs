// Topic: Basic arithmetic

/**
    Program requirements:
    * Displays the result of the sum of the two numbers
    
    Notes:
    * Use a function to add two numbers together
    * Use a function to dsiplay the result
    * Use the "{:?}" token in the println macro to display the result
**/

// add two numbers
fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

// display the result
fn display_result(result: i32) {
    println!("{:?}", result);
}

fn main() {
    let sum = add(5, 10);
    display_result(sum);
}