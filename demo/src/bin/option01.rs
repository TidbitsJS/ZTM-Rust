struct Customer {
    age: Option<i32>,
    email: String,
}

fn main() {
    let customer_vector = vec![
        Customer {
            age: Some(22), email: "mark@gmail.com".to_owned()
        },
        Customer {
            age: None, email: "becky@gmail.com".to_owned()
        }
    ];

    for customer in customer_vector {
        match customer.age {
            Some(age) => println!("{:?}", age),
            None => println!("No age"),
        }
    } 
}
