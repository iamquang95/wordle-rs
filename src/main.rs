#[macro_use]
extern crate lazy_static;

use std::io;
use crate::game::Game;
use crate::game_ui::GameUI;
use crate::words_lib::WordsLib;
use anyhow::Result;

mod game;
mod game_ui;
mod words_lib;

lazy_static! {
    static ref WORDS_LIB: WordsLib =
        WordsLib::load("./assets/5-letters-word.data").expect("Failed to load words lib");
}

fn main() -> Result<()> {
    let mut game = Game::new(5)?;
    loop {
        println!("{}", GameUI::display_board(&game));
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        match game.guess(buffer.as_str()) {
            Ok(state) => {
                dbg!(state);
            },
            Err(err) => {
                dbg!(err);
            }
        }
    }
}
