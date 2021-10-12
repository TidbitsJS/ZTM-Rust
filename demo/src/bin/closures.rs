fn add_fn(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum = add_fn(1, 2);

    let add_closure = |a: i32, b: i32| -> i32 {
        a + b
    };

    let add_closure_short = |a, b| a + b;


    let sum_closure = add_closure(1, 2);
    let sum_closure_short = add_closure_short(1, 2);

    println!("{:?} -- {:?} -- {:?}", sum, sum_closure, sum_closure_short);
}
