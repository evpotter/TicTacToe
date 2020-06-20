use std::io;
use std::process;

fn main() {
    TicTacToe::start();
}

struct TicTacToe {
    player: u8,
    grid: [char; 9],
}

impl TicTacToe {
    fn start() {
        let game: TicTacToe = TicTacToe {
            player: 1,
            grid: [' '; 9],
        };

        game.play();
    }

    fn play(mut self) {
        println!("Welcome to TicTacToe!");
        println!("Players will alternate turns.");
        println!(
            "Each turn the player will enter the position they want to play ranging from 1 - 9."
        );
        println!("The game can be exited by entering 'quit'");
        println!("A help menu can be shown by entering 'help'");

        loop {
            self.print();
            let pos: usize = self.get_input();
            self.set_indice(pos);
            if self.check_win() {
                println!("Player {} wins! Play again!", self.player);
                self.reset();
                continue;
            }
            self.change_player();
        }
    }

    fn get_input(&self) -> usize {
        loop {
            let mut pos = String::new();
            println!("Player {} enter your position:", self.player);
            io::stdin()
                .read_line(&mut pos)
                .expect("Failed to read line");

            match pos.trim() {
                "quit" => process::exit(0),
                "help" => {
                    TicTacToe::print_help();
                    continue;
                }
                val => match val.parse() {
                    Ok(num) if num > 0 && num < 10 => break num,
                    _ => {
                        println!("The input was not valid");
                        continue;
                    }
                },
            }
        }
    }

    fn print(&self) {
        let mut output: String = String::new();
        for (index, elm) in self.grid.iter().enumerate() {
            let tmp = match index {
                2 | 5 => format!(" {} \n - | - | - \n", elm),
                8 => format!(" {} ", elm),
                _ => format!(" {} |", elm),
            };
            output = format!("{}{}", output, tmp);
        }
        println!("{}", output);
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
        self.grid[indice - 1] = if self.player == 1 { 'x' } else { 'o' };
    }

    fn reset(&mut self) {
        self.grid = [' '; 9];
        self.player = 1;
    }

    fn change_player(&mut self) {
        self.player = if self.player == 1 { 2 } else { 1 };
    }

    fn print_help() {
        println!("Enter 1 - 9 to play, 'help' to print this, and 'quit' to exit");
    }
}

