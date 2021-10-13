fn main() {
    let range = 1..=3;
    let range_exclude = 1..4;

    for num in 1..4 {
        println!("{:?}", num);
    }

    for ch in 'a'..='f' {
        println!("{:?}", ch);
    }

    println!("{:?}", range);
    println!("{:?}", range_exclude);
}
