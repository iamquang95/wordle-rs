use crate::{WordsLib, WORDS_LIB};
use rand::Rng;

use anyhow::Result;

pub struct Game {
    word: String,
    num_guesses: usize,
    state: GameState,
}

impl Game {
    pub fn new(num_guesses: usize) -> Result<Game> {
        let word = WORDS_LIB.random_word()?;
        let state = GameState {
            guesses: vec![],
            state: State::Playing { turn: 0 },
        };
        Ok(Game {
            word,
            num_guesses,
            state,
        })
    }
}

struct GameState {
    guesses: Vec<String>,
    state: State,
}

enum State {
    Playing { turn: usize },
    Lose,
    Win { score: usize },
}
