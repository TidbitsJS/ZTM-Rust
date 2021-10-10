struct LineItem {
    name: String,
    count: i32
}

fn print_name(name: &str) {
    println!("Name: {:?}", name);
}

fn main() {
    let receipt = vec![
        LineItem {
            name: "Apple".to_owned(),
            count: 3
        },
        LineItem {
            name: String::from("Coffee"),
            count: 2
        }
    ];

    for item in receipt {
        print_name(&item.name);
        println!("count: {:?}", item.count);
    }
}
