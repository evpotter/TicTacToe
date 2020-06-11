use std::io;
use std::process;

fn main() {
    println!("Welcome to TicTacToe!");
    println!("Players will alternate turns.");
    println!("Each turn the player will enter the position they want to play ranging from 1 - 9.");
    println!("The game can be exited by entering 'quit'");
    println!("A help menu can be shown by entering 'help'");

    start_game();
}

fn start_game() {
    let mut arr: [char; 9] = [' '; 9];
    let mut player: u8 = 1;

    loop {
        print_grid(arr);
        let pos: usize = get_input_from_usr(player);
        let mark = if player == 1 { 'x' } else { 'o' };
        arr = set_indice(arr, pos, mark);
        if check_win(arr) {
            println!("Player {} wins! Play again!", player);
            arr = [' '; 9];
            player = 1;
            continue;
        }
        player = if player == 1 { 2 } else { 1 };
    }
}

fn check_win(arr: [char; 9]) -> bool {
    // loop 3 times checking rows and cols
    for set in 0..3 {
        // check row
        let h1 = arr[(3 * set)];
        let h2 = arr[1 + (3 * set)];
        let h3 = arr[2 + (3 * set)];
        if h1 != ' ' && h1 == h2 && h1 == h3 {
            return true;
        }

        // check column
        let v1 = arr[set];
        let v2 = arr[3 + set];
        let v3 = arr[6 + set];
        if v1 != ' ' && v1 == v2 && v1 == v3 {
            return true;
        }
    }

    // check diagonals
    let d1 = arr[0];
    let d2 = arr[2];
    let d3 = arr[4];
    let d4 = arr[6];
    let d5 = arr[8];
    if d1 != ' ' && d1 == d3 && d1 == d5 {
        return true;
    }
    if d2 != ' ' && d2 == d3 && d2 == d4 {
        return true;
    }
    false
}

fn get_input_from_usr(usr: u8) -> usize {
    loop {
        let mut pos = String::new();
        println!("Player {} enter your position:", usr);
        io::stdin()
            .read_line(&mut pos)
            .expect("Failed to read line");

        match pos.trim() {
            "quit" => process::exit(0),
            "help" => {
                print_help();
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

fn print_help() {
    println!("Enter 1 - 9 to play, 'help' to print this, and 'quit' to exit");
}

fn set_indice(arr: [char; 9], indice: usize, value: char) -> [char; 9] {
    let mut tmp = arr;
    tmp[indice - 1] = value;
    tmp
}

fn print_grid(arr: [char; 9]) {
    let mut output: String = String::new();
    for (index, elm) in arr.iter().enumerate() {
        let tmp = match index {
            2 | 5 => format!(" {} \n - | - | - \n", elm),
            8 => format!(" {} ", elm),
            _ => format!(" {} |", elm),
        };
        output = format!("{}{}", output, tmp);
    }
    println!("{}", output);
}
