enum Direction {
    Left,
    Right,
    Up,
    Down
}

fn display_direction (dir: Direction) {
    match dir {
        Direction::Left => println!("Going left"),
        Direction::Right => println!("Going right"),
        Direction::Up => println!("Going up"),
        Direction::Down => println!("Going down"),
    }
}

fn main() {
    display_direction(Direction::Left);
    display_direction(Direction::Down);
}