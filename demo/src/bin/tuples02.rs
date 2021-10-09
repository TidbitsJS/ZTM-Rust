fn main() {
    let coord = (2, 3);
    println!("{:?}, {:?}", coord.0, coord.1);

    let (x, y) = coord;
    println!("{:?}, {:?}", x, y);

    let (name, age) = ("John", "21");
    println!("{:?}, {:?}", name, age);

    let favorites = ("Purple", 14, "London", "Pizza", "Enola Holmes", "Home");
    
    let state = favorites.2;
    let place = favorites.5;
}