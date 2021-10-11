// Topic: Result

/**
    Program requirements:
    * Determine if a customer is able to make a restricted purchase
    * Restricted purchases require that the age of the customer 
        is at least 21

    Notes:
    * Use a struct to stor at least the age of a customer
    * Use a function to determine if a customer can make a restricted purchase
    * Return a result from the function
    * The Err variant should detail the reason why they cannot make a purchase
**/

#[derive(Debug)]
struct Customer {
    age: i32
}

fn purchase(input: i32) -> Result<Customer, String> {
    if input >= 21 {
        Ok(Customer { age: input })
    } else {
        Err(String::from("Customer is not old enough"))
    }
}

fn main() {
    let jack: Result<Customer, _> = purchase(23);
    println!("Can purchase? {:?}", jack);

    let joe: Result<Customer, _> = purchase(13);
    println!("Can purchase? {:?}", joe);
}
