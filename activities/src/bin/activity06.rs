// Topic: Looping using the while statement

/**
    Program requirements:
    * Counts down from 5 to 1, displays the countdown
        in the terminal, then prints "done!" when complete.

    Notes:
    * Use a mutable integer varaible
    * Use a while statment
    * Print the variable within the while loop
    * Do not use break to exit the loop
**/

fn main() {
    let mut i : i32 = 5;
    
    while i > 0 {
        println!("{:?}", i);
        i -= 1;
    }

    println!("done!")
}
