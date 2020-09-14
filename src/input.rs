use std::io;

pub enum Input {
    Quit,
    Help,
    History,
    Some(usize),
    None,
}

pub fn get_input() -> Input {
    let mut pos = String::new();
    io::stdin()
        .read_line(&mut pos)
        .expect("Failed to read line");

    match pos.trim() {
        "quit" => Input::Quit,
        "help" => Input::Help,
        "history" => Input::History,
        val => match val.parse() {
            Ok(num) => Input::Some(num),
            _ => Input::None,
        },
    }
}

