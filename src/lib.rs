mod board;
mod input;
mod player;
use input::Input;

pub fn play() {
    println!("Welcome to TicTacToe!");
    println!("Players will alternate turns.");
    println!("Each turn the player will enter the position they want to play ranging from 1 - 9.");
    println!("Every move of the current game can be seen by entering 'history'");
    println!("The game can be exited by entering 'quit'");
    println!("A help menu can be shown by entering 'help'");

    let mut board = board::new();
    loop {
        board.print();
        println!("Player {} enter your position:", board.get_player());
        let input: Input = input::get_input();
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

fn print_help() {
    println!("Enter 1 - 9 to play, 'help' to print this, and 'quit' to exit");
}
