// Topic: Organizing similar data using structs

/**
    Program requirements:
    * Print the flavor of a drink and it's fluid ounces.

    Notes:
    * Use an enum to create different flavors of drinks.
    * Use a struct to store drink flavor and fluid ounce information
    * Use a function to print out the drink flavor and ounces
    * Use a match expression to print the drink flavor
**/

// enum for drink flavors
enum Flavor {
    Mint,
    Strawberry,
    Vanilla,
    Coffee,
}

// struct for drink information
struct Drink {
    flavor: Flavor,
    ounces: f64,
}

// display drink information
fn display_drink_info(drink: Drink) {
    match drink.flavor {
        Flavor::Mint => println!(" Mint {:?}oz",  drink.ounces),
        Flavor::Strawberry => println!(" Strawberry {:?}oz",  drink.ounces),
        Flavor::Vanilla => println!(" Vanilla {:?}oz",  drink.ounces),
        Flavor::Coffee => println!(" Coffee {:?}oz",  drink.ounces),
    }
}

fn main() {
    let strawberry_shake = Drink {
        flavor: Flavor::Strawberry,
        ounces: 16.45,
    };
    
    let cold_coffee = Drink {
        flavor: Flavor::Coffee,
        ounces: 10.11
    };
    
    let mint = Drink {
        flavor: Flavor::Mint,
        ounces: 19.23
    };
    
    let vanilla = Drink {
        flavor: Flavor::Vanilla,
        ounces: 23.78
    };

    display_drink_info(strawberry_shake);
    display_drink_info(cold_coffee);
    display_drink_info(mint);
    display_drink_info(vanilla);
}
