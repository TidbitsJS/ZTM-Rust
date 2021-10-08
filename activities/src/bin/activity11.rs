// Topic: Ownership

/**
    Program requirements:
    * Print out the quantity and id number of a grocery item.

    Notes:
    * Use a struct for the grocery item
    * Use two i32 fields for the quantity and id number
    * Create a function to display the quantity, with the struct as a parameter
    * Create a function to display the id number, with the struct as a parameter
**/

// struct for the grocery item
struct Grocery {
    quantity: i32,
    id: i32,
}

// display the quantity
fn display_quantity(item: &Grocery) {
    println!("Quantity {:?}", item.quantity);
}

// display the id number
fn display_id(item: &Grocery) {
    println!("Id {:?}", item.id);
}

fn main() {
    let chocolate = Grocery {
        quantity: 10,
        id: 11,
    };

    display_id(&chocolate);
    display_quantity(&chocolate);
}
