#[macro_use]
extern crate lazy_static;

use crate::game::Game;
use crate::game_ui::GameUI;
use crate::words_lib::WordsLib;
use anyhow::Result;
use std::io;
use std::io::{stdout, Write};
use termion::screen::AlternateScreen;

mod game;
mod game_ui;
mod words_lib;

lazy_static! {
    static ref WORDS_LIB: WordsLib =
        WordsLib::load("./assets/5-letters-word.data").expect("Failed to load words lib");
}

fn main() -> Result<()> {
    let mut game = Game::new(5)?;
    let mut screen = AlternateScreen::from(stdout());
    loop {
        write!(screen, "{}", termion::cursor::Save);
        write!(screen, "{}\n", GameUI::display_header(&game));
        write!(screen, "{}\n", GameUI::display_board(&game));
        write!(screen, "{}", termion::clear::CurrentLine);
        write!(screen, "{}", termion::cursor::BlinkingUnderline);
        screen.flush()?;
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        match game.guess(buffer.as_str()) {
            Ok(state) => {
                dbg!(state);
            }
            Err(err) => {
                dbg!(err);
            }
        }
        write!(screen, "{}", termion::cursor::Restore);
    }
}
