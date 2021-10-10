// Topic: Advanced match

/**
    Program requirements:
    * Print out a list of tickets and their information for an event.
    * Tickets can be Backstage, Vip, and Standard.
    * Backstage and Vip tickets include the ticket holder's name.
    * All tickets include the price.

    Notes:
    * Use an enum for the tickets with data associated with each variant.
    * Create one of each ticket and place into a vector.
    * Use a match expression while iterating the vector to print the ticket info.
**/

// enum for tickets
enum Ticket {
    Standard(i32),
    Vip(i32, String),
    Backstage(f64, String),
}

fn main() {
    let tickets_vector = vec![
        Ticket::Standard(10),
        Ticket::Vip(20, "John".to_owned()),
        Ticket::Backstage(30.0, String::from("Jack")),
        Ticket::Standard(40),
        Ticket::Vip(50, "Jane".to_owned()),
        Ticket::Backstage(60.0, String::from("Joe")),
    ];

    for ticket in tickets_vector {
        match ticket {
            Ticket::Standard(price) => println!("Standard ticket {:?}", price),
            Ticket::Vip(price, name) => println!("Vip ticket {:?} for {:?}", price, name),
            Ticket::Backstage(price, name) => println!("Backstage ticket {:?} for {:?}", price, name)
        }
    }
}
