use std::io;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Rover {
    x: i32,
    y: i32,
}

impl Rover {
    fn new() -> Rover {
        Rover { x: 0, y: 0 }
    }

    fn move_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

fn main() {
    let mut rover = Rover::new();

    loop {
        println!("Enter direction (u/d/l/r) or 'q' to quit:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "u" => rover.move_direction(Direction::Up),
            "d" => rover.move_direction(Direction::Down),
            "l" => rover.move_direction(Direction::Left),
            "r" => rover.move_direction(Direction::Right),
            "q" => break,
            _ => println!("Invalid input! Please enter 'u', 'd', 'l', 'r' or 'q'."),
        }

        let position = rover.position();
        println!("Rover's position: {:?}", position);
    }
}
