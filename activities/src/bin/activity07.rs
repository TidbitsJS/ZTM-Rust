// Topic: Working with an enum

/**
    Program requirements:
    * Prints the name of a color to the terminal

    Notes:
    * Use an enum with color names as variants
    * Use a function to print the color name
    * The function must use the enum as a parameter
    * Use a match expression to determine which color name to print
**/

// enum color 
enum Colors {
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
    Orange,
    Black,
    White
}

// display color to terminal
fn display_color(color: Colors) {
    match color {
        Colors::Red => println!("red"),
        Colors::Green => println!("green"),
        Colors::Blue => println!("blue"),
        Colors::Yellow => println!("yellow"),
        Colors::Purple => println!("purple"),
        Colors::Orange => println!("orange"),
        Colors::Black => println!("black"),
        Colors::White => println!("white"),
    }
}

fn main() {
    display_color(Colors::Orange);
    display_color(Colors::Green);
    display_color(Colors::Purple);
}
