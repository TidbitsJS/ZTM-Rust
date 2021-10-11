// Topic: Browsing standard library documentation

/**
    Program requirements:
    * Print a String in lowecase and uppercase

    Notes:
    * Utilize standard library functionality to
        transform the string to lowecase and uppercase
    * Use 'rustup doc' in a terminal to open the standard library docs
    * Navigate to the API documentation section
    * Search for functionality to transform a string ( or str )
        to uppercase and lowercase
    * Try searching for: to_uppercase, to_lowercase
**/

fn main() {
    let my_string = "Tidbits";

    println!("{:?}", my_string.to_lowercase());
    println!("{:?}", my_string.to_uppercase());
}
