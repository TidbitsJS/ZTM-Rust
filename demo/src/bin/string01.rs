fn print_it(data: &str) {
    println!("{:?}", data);
}

fn main() {
    print_it("a string slice");

    let owned_string = "owned string".to_owned();
    let another_owned = String::from("another owned string");

    print_it(&owned_string);
    print_it(&another_owned);
}
