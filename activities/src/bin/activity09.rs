// Topic: Data management using tuples

/**
    Program requirements:
    * Print whether the y-value of a cartesian coordinate is
        greater than 5, less than 5, or equal to 5.

    Notes:
    * Use a function that returns a tuple
    * Destructure the return value into two variables
    * Use an if..else if..else block to determine what to print
**/

// returns a tuple
fn coords(num1:i32, num2:i32) -> (i32, i32) {
    (num1, num2)
}

fn main() {
    let (_x, y) = coords(5, 12);
    
    if y > 5 {
        println!("{:?} is greater than 5", y);
    } else if y < 5 {
        println!("{:?} is less than 5", y);
    } else {
        println!("{:?} is equal to 5", y)
    }
}
