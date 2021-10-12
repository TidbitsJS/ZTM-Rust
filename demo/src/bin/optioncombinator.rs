fn main() {
    let a: Option<i32> = Some(1);
    dbg!(a);

    let a_is_some = a.is_some();
    dbg!(a_is_some);

    let a_is_none = a.is_none();
    dbg!(a_is_none);

    let a_mapped = a.map(|num| num + 1);
    dbg!(a_mapped);

    let a_filtered = a.filter(|num| num == &1);
    dbg!(a_filtered);
    
    let a_or_else = a.or_else(|| Some(5));
    dbg!(a_or_else);
    
    let a_unwrapped = a.unwrap_or_else(|| 7);
    dbg!(a_unwrapped);


    let b: Option<i32> = None;
    dbg!(b);

    let b_is_some = b.is_some();
    dbg!(b_is_some);

    let b_is_none = b.is_none();
    dbg!(b_is_none);

    let b_mapped = b.map(|num| num + 1);
    dbg!(b_mapped);

    let b_filtered = b.filter(|num| num == &1);
    dbg!(b_filtered);
    
    let b_or_else = b.or_else(|| Some(5));
    dbg!(b_or_else);
    
    let b_unwrapped = b.unwrap_or_else(|| 7);
    dbg!(b_unwrapped);
}
