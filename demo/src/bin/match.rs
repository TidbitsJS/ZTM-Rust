enum Discount {
    Percent(i32),
    Flat(i32)
}

struct Ticket  {
    event: String,
    price: i32
}

fn main() {
    let n = 3;

    match n {
        3 => println!("three"),
        other => println!("number: {:?}", other)
    }

    let flat = Discount::Flat(11);
    match flat {
        Discount::Flat(20) => println!("Flat 20% discount"),
        Discount::Flat(amount) => println!("Flat discount of {:?}", amount),
        _ => println!("No discount"),
    }

    let concert = Ticket {
        event: "concert".to_owned(),
        price: 50,
    };

    match concert {
        Ticket { price: 50, event } => println!("Event @50.0 = {:?}", event),
        Ticket { price, .. } => println!("Price: ${:?}", price),
    }
}
