use std::fmt;
use std::io;
use std::process;

fn main() {
    TicTacToe::start();
}

enum UserInput {
    Quit,
    Help,
    History,
    Some(usize),
    None,
}

enum Player {
    P1,
    P2,
}

impl Player {
    fn get_mark(&self) -> char {
        match self {
            Player::P1 => 'x',
            Player::P2 => 'o',
        }
    }

    fn get_next(&self) -> Player {
        match self {
            Player::P1 => Player::P2,
            Player::P2 => Player::P1,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Player::P1 => 1,
            Player::P2 => 2,
        };
        write!(f, "{}", output)
    }
}

struct TicTacToe {
    player: Player,
    grid: [char; 9],
    history: Vec<[char; 9]>,
}

impl TicTacToe {
    fn start() {
        let game: TicTacToe = TicTacToe {
            player: Player::P1,
            grid: [' '; 9],
            history: vec![[' '; 9]],
        };

        game.play();
    }

    fn play(mut self) {
        println!("Welcome to TicTacToe!");
        println!("Players will alternate turns.");
        println!(
            "Each turn the player will enter the position they want to play ranging from 1 - 9."
        );
        println!("Every move of the current game can be seen by entering 'history'");
        println!("The game can be exited by entering 'quit'");
        println!("A help menu can be shown by entering 'help'");

        loop {
            print_grid(self.grid);
            let input: UserInput = self.get_input();
            self.execute_input(input);
        }
    }

    fn get_input(&self) -> UserInput {
        let mut pos = String::new();
        println!("Player {} enter your position:", self.player);
        io::stdin()
            .read_line(&mut pos)
            .expect("Failed to read line");

        match pos.trim() {
            "quit" => UserInput::Quit,
            "help" => UserInput::Help,
            "history" => UserInput::History,
            val => match val.parse() {
                Ok(num) if self.valid_indice(num) => UserInput::Some(num),
                _ => UserInput::None,
            },
        }
    }

    fn valid_indice(&self, indice: usize) -> bool {
        match indice {
            _ if indice > 0 && indice < 10 && self.grid[indice - 1] == ' ' => true,
            _ => false,
        }
    }

    fn execute_input(&mut self, input: UserInput) {
        match input {
            UserInput::Quit => process::exit(0),
            UserInput::Help => TicTacToe::print_help(),
            UserInput::History => self.show_history(),
            UserInput::None => println!("The input was not valid"),
            UserInput::Some(indice) => self.execute_turn(indice),
        }
    }

    fn execute_turn(&mut self, indice: usize) {
        self.set_indice(indice);
        if self.check_win() {
            println!("Player {} wins! Play again!", self.player);
            self.reset();
        } else {
            self.push_history();
            self.change_player();
        }
    }

    fn check_win(&self) -> bool {
        // loop 3 times checking rows and cols
        for set in 0..3 {
            // check row
            let h1 = self.grid[(3 * set)];
            let h2 = self.grid[1 + (3 * set)];
            let h3 = self.grid[2 + (3 * set)];
            if h1 != ' ' && h1 == h2 && h1 == h3 {
                return true;
            }

            // check column
            let v1 = self.grid[set];
            let v2 = self.grid[3 + set];
            let v3 = self.grid[6 + set];
            if v1 != ' ' && v1 == v2 && v1 == v3 {
                return true;
            }
        }

        // check diagonals
        let d1 = self.grid[0];
        let d2 = self.grid[2];
        let d3 = self.grid[4];
        let d4 = self.grid[6];
        let d5 = self.grid[8];
        if d1 != ' ' && d1 == d3 && d1 == d5 {
            return true;
        }
        if d2 != ' ' && d2 == d3 && d2 == d4 {
            return true;
        }
        false
    }

    fn set_indice(&mut self, indice: usize) {
        self.grid[indice - 1] = self.player.get_mark();
    }

    fn reset(&mut self) {
        self.grid = [' '; 9];
        self.player = Player::P1;
        self.history = vec![[' '; 9]]
    }

    fn change_player(&mut self) {
        self.player = self.player.get_next();
    }

    fn show_history(&self) {
        for (turn, grid) in self.history.iter().enumerate() {
            print_grid(*grid);
            println!("Turn: {}", turn);
        }
        println!();
    }

    fn push_history(&mut self) {
        self.history.push(self.grid);
    }

    fn print_help() {
        println!("Enter 1 - 9 to play, 'help' to print this, and 'quit' to exit");
    }
}

fn print_grid(grid: [char; 9]) {
    let mut output: String = String::new();
    for (index, elm) in grid.iter().enumerate() {
        let tmp = match index {
            2 | 5 => format!(" {} \n - | - | - \n", elm),
            8 => format!(" {} ", elm),
            _ => format!(" {} |", elm),
        };
        output = format!("{}{}", output, tmp);
    }
    println!("{}", output);
}
