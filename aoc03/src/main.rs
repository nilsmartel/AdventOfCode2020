mod parse;

pub struct Cable {
    direction: Direction,
    path: isize,
}

impl Cable {
    pub fn new(direction: Direction, path: isize) -> Cable {
        Cable { direction, path }
    }
}

// Only two Directions.
// Left/Down will be treated as negative path.
enum Direction {
    Up,
    Right,
}

fn main() {
    println!("Hello, world!");
}
