mod greet {
    pub fn hello() {
        println!("Hello, world!");
    }
    
    pub fn goodbye() {
        println!("Goodbye, world!");
    }
}

mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    pub fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}

fn main() {

    use math::*;

    greet::hello();
    
    println!("{:?}", add(1, 2));
    println!("{:?}", sub(1, 2));

    greet::goodbye();
}
