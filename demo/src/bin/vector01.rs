fn main() {
    let my_vector = vec![1, 2, 3, 4, 5];

    let mut my_numbers = Vec::new();
    my_numbers.push(1);
    my_numbers.push(2);
    my_numbers.push(3);
    my_numbers.push(9);
    my_numbers.push(5);
    my_numbers.pop();
    my_numbers.len();

    println!("{:?} {:?}", my_numbers, my_numbers.len());

    let two = my_numbers[1];
    println!("Two {:?}", two);

    for num in my_vector {
        println!("{:?}", num);
    }
}
