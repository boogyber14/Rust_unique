use std::io;

#[derive(Debug)]
struct Player {
    x: i32,
    y: i32,
}

impl Player {
    fn new() -> Player {
        Player { x: 0, y: 0 }
    }

    fn move_up(&mut self) {
        self.y -= 1;
    }

    fn move_down(&mut self) {
        self.y += 1;
    }

    fn move_left(&mut self) {
        self.x -= 1;
    }

    fn move_right(&mut self) {
        self.x += 1;
    }
}

fn main() {
    let mut player = Player::new();
    let goal_x = 2;
    let goal_y = 2;

    loop {
        println!("Current Position: ({}, {})", player.x, player.y);
        println!("Enter your move (up, down, left, right, or quit):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim().to_lowercase();

        match input.as_str() {
            "up" => player.move_up(),
            "down" => player.move_down(),
            "left" => player.move_left(),
            "right" => player.move_right(),
            "quit" => {
                println!("Exiting the game. Bye!");
                break;
            }
            _ => {
                println!("Invalid input! Please enter 'up', 'down', 'left', 'right', or 'quit'.");
                continue;
            }
        }

        if player.x == goal_x && player.y == goal_y {
            println!("Congratulations! You reached the goal position!");
            break;
        }
    }
}
