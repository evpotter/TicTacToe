mod board;
mod input;

use board::Board;
use input::{Input, Inputs};
use std::error::Error;
use std::io::Stdout;
use termion::raw::{IntoRawMode, RawTerminal};
use tui::layout::{Constraint, Layout};
use tui::widgets::Paragraph;
use tui::{
    backend::TermionBackend,
    style::{Color, Style},
    widgets::{Block, Borders},
    Terminal,
};

pub fn play() -> Result<(), Box<dyn Error>> {
    let stdout = std::io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let inputs = Inputs::new();
    let mut board = Board::new();
    let mut turn_text: Option<String> = None;
    loop {
        let board_str = board.print();
        let move_text = turn_text.unwrap_or_default();
        draw_terminal(&mut terminal, board_str, move_text)?;

        let input = inputs.next()?;
        turn_text = match input {
            Input::Quit => break,
            Input::Help => None,
            Input::History => Some(board.show_history()),
            Input::None => Some(String::from("The input was not valid")),
            Input::Some(indice) => board.execute_input(indice),
        };
    }

    Ok(())
}

fn draw_terminal(
    term: &mut Terminal<TermionBackend<RawTerminal<Stdout>>>,
    board_str: String,
    move_text: String,
) -> Result<(), Box<dyn Error>> {
    let welcome_text = String::from(
        "\
Welcome to TicTacToe!
Players will alternate turns.
Each turn the player will enter the position they want to play ranging from 1 - 9.
Every move of the current game can be seen by entering 's'
The game can be exited by entering 'q'",
    );

    term.draw(|f| {
        let size = f.size();
        let main_block = Block::default()
            .title("TicTacToe")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black).fg(Color::White));
        f.render_widget(main_block, size);
        let main_chunks = Layout::default()
            .direction(tui::layout::Direction::Horizontal)
            .margin(5)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(size);
        let messaging_chunks = Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .margin(5)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(main_chunks[0]);
        let game_chunks = Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .margin(5)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(main_chunks[1]);
        let paragraph_1 = Paragraph::new(welcome_text.clone())
            .style(Style::default().bg(Color::Black).fg(Color::White))
            .block(Block::default())
            .alignment(tui::layout::Alignment::Center);
        f.render_widget(paragraph_1, messaging_chunks[0]);
        let paragraph_1_2 = Paragraph::new(move_text)
            .style(Style::default().bg(Color::Black).fg(Color::White))
            .block(Block::default())
            .alignment(tui::layout::Alignment::Center);
        f.render_widget(paragraph_1_2, messaging_chunks[1]);
        let paragraph_2 = Paragraph::new(board_str)
            .style(Style::default().bg(Color::Black).fg(Color::White))
            .block(Block::default())
            .alignment(tui::layout::Alignment::Center);
        f.render_widget(paragraph_2, game_chunks[0]);
    })?;

    Ok(())
}
