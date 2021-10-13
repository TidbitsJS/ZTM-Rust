fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let mut plus_one = vec![];
    for num in &numbers {
        plus_one.push(num + 1);
    }

    let plus_one_iter: Vec<_> = numbers
        .iter()
        .map(|num| num + 1)
        .collect();

    let new_numbers: Vec<_> = numbers
        .iter()
        .filter(|num| num <= &&3)
        .collect();

    let find_me: Option<_> = numbers
        .iter()
        .find(|num| num == &&3);

    let count = numbers
        .iter()
        .count();

    let last = numbers
        .iter()
        .last();    

    let min = numbers
        .iter()
        .min();
    
    let max = numbers
        .iter()
        .max();

    println!("Plus one {:?}", plus_one);
    println!("Plus one iterator {:?}", plus_one_iter);
    println!("Filter numbers {:?}", new_numbers);
    println!("Find number {:?}", find_me);
    println!("Count {:?}", count);
    println!("Last {:?}", last);
    println!("Min {:?}", min);
    println!("Max {:?}", max);
}
