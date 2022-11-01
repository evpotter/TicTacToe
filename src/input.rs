use std::{io, sync::mpsc, thread};

use termion::{event::Key, input::TermRead};

pub enum Input {
    Quit,
    Help,
    History,
    Some(usize),
    None,
}

pub struct Inputs {
    rx: mpsc::Receiver<Input>,
    input_handle: thread::JoinHandle<()>,
}

fn get_input(key: Key) -> Input {
    match key {
        Key::Char('q') => Input::Quit,
        Key::Char('h') => Input::Help,
        Key::Char('s') => Input::History,
        Key::Char(x) if x.is_digit(10) => match x.to_digit(10) {
            Some(num) if num != 0 => Input::Some(num as usize),
            _ => Input::None,
        },
        _ => Input::None,
    }
}

impl Inputs {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let input_handle = thread::spawn(move || {
            let stdin = io::stdin();
            for input in stdin.keys().flatten() {
                let i = get_input(input);
                if let Err(e) = tx.send(i) {
                    eprintln!("Error sending input: {}", e);
                    return;
                }
            }
        });

        Inputs { rx, input_handle }
    }

    pub fn next(&self) -> Result<Input, mpsc::RecvError> {
        self.rx.recv()
    }
}
