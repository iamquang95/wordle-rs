use crate::words_lib::WordsLib;
use anyhow::Result;

mod words_lib;

fn main() -> Result<()> {
    let words_lib = WordsLib::load("./assets/5-letters-word.data")?;
    dbg!(words_lib.words_size());
    println!("Hello, world!");
    Ok(())
}
