// Topic: Strings

/**
    Program requirements:
    * Print out the name and favorite colors of people aged 10 and under.

    Notes:
    * Use a struct for a persons age, name, and favorite color.
    * The color and name should be stored as a String.
    * Create and store at least 3 people in a vector.
    * Iterate through the vector using a for...in loop.
    * Use an if expression to determine which person's info should be printed.
    * The name and colors should be printed using a function.
**/

// struct for a person
struct Person {
    age: i32,
    name: String,
    favorite_color: String
}

// function to print out person's info
fn print_it(name: &str, color: &str) {
    println!("{}'s favorite color is {}", name, color);
}

fn main() {

    // create a vector of people
    let people_vector = vec![
        Person {
            age: 10,
            name: "John Doe".to_owned(),
            favorite_color: "Blue".to_owned()
        },
        Person {
            age: 12,
            name: "Suzan Walker".to_owned(),
            favorite_color: "Green".to_owned()
        },
        Person {
            age: 8,
            name: String::from("Peter Parker"),
            favorite_color: String::from("Red")
        },
        Person {
            age: 5,
            name: String::from("Mary Jane"),
            favorite_color: String::from("Purple")
        }
    ];

    // iterate through the vector
    for info in people_vector {
        if info.age <= 10 {
            println!("Age: {:?}", info.age);
            print_it(&info.name, &info.favorite_color);
        }
    }
}
