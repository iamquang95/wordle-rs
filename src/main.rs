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
    let mut game = Game::new(5)?;
    dbg!(game.guess("HEllo\n"));
    dbg!(game.guess("HEllo\n"));
    dbg!(game.guess("HEllo\n"));
    dbg!(game.guess("HEllo\n"));
    dbg!(game.guess("HEllo\n"));
    dbg!(game.current_result());
    println!("Hello, world!");
    Ok(())
}
