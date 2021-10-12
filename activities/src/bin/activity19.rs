// Topic: HashMap

/**
    Program requirements:
    * Print the name and number of items in stock for a furniture store.
    * If the number of items is 0, print "Out of Stock" instead of 0.
    * The store has:
        * 5 Chairs
        * 3 Beds
        * 2 Tables
        * 0 Couches
    * Print the total number of items in stock.

    Notes:
    * Use a HashMap for the furniture store stock.
**/

// import the HashMap library
use std::collections::HashMap;

#[derive(Debug)]
struct Store {
    name: String,
    count: i32
}

fn main() {
    let mut store = HashMap::new();
    store.insert(1, Store { name: "Chairs".to_string(), count: 5 });
    store.insert(2, Store { name: "Beds".to_string(), count: 3 });
    store.insert(3, Store { name: "Tables".to_string(), count: 2 });
    store.insert(4, Store { name: "Couches".to_string(), count: 0 });

    let mut total_stock = 0;

    for (_, item) in store.iter() {
        total_stock += item.count;
        match item.count {
            0 => println!("{:?} is out of stock", item.name),
            _ => println!("{:?} has {:?} items in stock", item.name, item.count),
        }
    }

    println!("Total stock: {:?}", total_stock);
}
