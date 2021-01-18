mod grid;
mod player;
use crate::board::grid::Grid;
use crate::board::player::Player;

pub struct Board {
    player: Player,
    grid: Grid,
}

impl Board {
    pub fn new() -> Board {
        Board {
            player: Player::P1,
            grid: grid::new(),
        }
    }

    pub fn execute_input(&mut self, indice: usize) {
        match indice {
            _ if self.grid.valid_indice(indice) => self.execute_turn(indice),
            _ => println!("Incorrect board position provided"),
        }
    }

    pub fn get_player(&self) -> &Player {
        &self.player
    }

    fn execute_turn(&mut self, indice: usize) {
        self.grid.set_indice(indice, self.player.get_mark());
        if self.grid.check_win() {
            println!("Player {} wins! Play again!", self.player);
            self.reset();
        } else {
            self.grid.push_history();
            self.change_player();
        }
    }

    fn reset(&mut self) {
        self.grid = grid::new();
        self.player = Player::P1;
    }

    fn change_player(&mut self) {
        self.player = self.player.get_next();
    }

    pub fn print(&self) {
        self.grid.print()
    }

    pub fn show_history(&self) {
        self.grid.show_history()
    }
}
