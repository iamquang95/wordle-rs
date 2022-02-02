#[macro_use]
extern crate lazy_static;

use crate::game::Game;
use crate::words_lib::WordsLib;
use anyhow::Result;

mod game;
mod words_lib;

lazy_static! {
    static ref WORDS_LIB: WordsLib =
        WordsLib::load("./assets/5-letters-word.data").expect("Failed to load words lib");
}

fn main() -> Result<()> {
    let game = Game::new(5)?;
    println!("Hello, world!");
    Ok(())
}
