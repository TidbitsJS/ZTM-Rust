struct GroceryItem {
    stock: i32,
    price: f64,
    name: String,
}

fn main() {
    let cereal = GroceryItem {
        stock: 10,
        price: 2.99,
        name: "Cereals".to_string(),
    };

    println!("stock: {:?}", cereal.name);
    println!("stock: {:?}", cereal.price);
    println!("stock: {:?}", cereal.stock);
}