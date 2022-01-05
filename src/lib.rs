mod board;
mod input;

use board::Board;
use input::Input;
use std::io::Write;

pub fn play() {
    println!("Welcome to TicTacToe!");
    println!("Players will alternate turns.");
    println!("Each turn the player will enter the position they want to play ranging from 1 - 9.");
    println!("Every move of the current game can be seen by entering 'history'");
    println!("The game can be exited by entering 'quit'");
    println!("A help menu can be shown by entering 'help'");

    let mut board = Board::new();
    loop {
        board.print();
        let input: Input = get_player_input(&board);
        match input {
            Input::Quit => {
                println!("Exiting game");
                break;
            }
            Input::Help => print_help(),
            Input::History => board.show_history(),
            Input::None => println!("The input was not valid"),
            Input::Some(indice) => board.execute_input(indice),
        }
    }
}

fn get_player_input(b: &Board) -> Input {
    print!("Player {} enter your position: ", b.get_player());
    match std::io::stdout().flush() {
        Ok(_) => input::get_input(),
        Err(e) => {
            println!("Error printing: {}", e);
            Input::Quit
        }
    }
}

fn print_help() {
    println!("1 - 9 to play,");
    println!("'history' to see the current games turns,");
    println!("'help' to print this,");
    println!("'quit' to exit");
}
