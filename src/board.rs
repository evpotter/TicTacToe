use crate::player::Player;

pub struct Board {
    player: Player,
    grid: [char; 9],
    history: Vec<[char; 9]>,
}

pub fn new() -> Board {
    Board {
        player: Player::P1,
        grid: [' '; 9],
        history: vec![[' '; 9]],
    }
}

impl Board {
    pub fn execute_input(&mut self, indice: usize) {
        match indice {
            _ if self.valid_indice(indice) => self.execute_turn(indice),
            _ => println!("Incorrect board position provided"),
        }
    }

    pub fn get_player(&self) -> &Player {
        &self.player
    }

    pub fn print(&self) {
        print_grid(&self.grid);
    }

    pub fn show_history(&self) {
        for (turn, grid) in self.history.iter().enumerate() {
            print_grid(grid);
            println!("Turn: {}", turn);
        }
        println!();
    }

    fn valid_indice(&self, indice: usize) -> bool {
        match indice {
            _ if indice > 0 && indice < 10 && self.grid[indice - 1] == ' ' => true,
            _ => false,
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

    fn push_history(&mut self) {
        self.history.push(self.grid);
    }
}

fn print_grid(grid: &[char; 9]) {
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
