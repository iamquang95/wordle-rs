use crate::{WordsLib, WORDS_LIB};
use rand::Rng;

use anyhow::{anyhow, Result};

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
            state: State::Playing,
        };
        Ok(Game {
            word,
            num_guesses,
            state,
        })
    }

    pub fn guess(&mut self, guessing_word: &str) -> Result<State> {
        let word = &guessing_word.trim().to_uppercase();
        self.check_valid_guess(word)?;
        let mut state = &mut self.state;
        state.guesses.push(word.to_owned());
        if self.word.eq(word) {
            state.state = State::Win
        } else if state.guesses.len() >= self.num_guesses {
            state.state = State::Lose
        }
        Ok(state.state.clone())
    }

    fn check_valid_guess(&self, word: &str) -> Result<()> {
        match &self.state.state {
            State::Lose | State::Win => return Err(anyhow!("Game ended")),
            State::Playing => (),
        }
        if !WORDS_LIB.contains(word) {
            return Err(anyhow!("Guessing word is not in the library"));
        }
        Ok(())
    }
}

struct GameState {
    guesses: Vec<String>,
    state: State,
}

#[derive(Clone, Debug)]
pub enum State {
    Playing,
    Lose,
    Win,
}
