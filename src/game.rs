use crate::{WordsLib, WORDS_LIB};
use rand::Rng;
use std::collections::HashMap;

use anyhow::{anyhow, Result};

pub struct Game {
    word: String,
    num_guesses: usize,
    state: GameState,
}

impl Game {
    pub fn new(num_guesses: usize) -> Result<Game> {
        let word = WORDS_LIB.random_word()?;
        dbg!(&word);
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

    pub fn current_result(&self) -> GameResult {
        let char_position = self
            .word
            .clone()
            .into_bytes()
            .iter()
            .enumerate()
            .map(|(idx, ch)| (*ch, idx))
            .collect::<HashMap<u8, usize>>();
        let judged_guesses = self
            .state
            .guesses
            .iter()
            .map(|guess| {
                let result = guess
                    .clone()
                    .into_bytes()
                    .iter()
                    .enumerate()
                    .map(|(idx, ch)| match char_position.get(ch) {
                        Some(w_idx) => {
                            if *w_idx == idx {
                                JudgedChar::Correct
                            } else {
                                JudgedChar::WrongPlace
                            }
                        }
                        None => JudgedChar::Wrong,
                    })
                    .collect::<Vec<JudgedChar>>();
                JudgedGuess {
                    guess: guess.clone(),
                    result,
                }
            })
            .collect::<Vec<JudgedGuess>>();
        GameResult {
            judged_guesses,
            state: self.state.state.clone(),
        }
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

#[derive(Debug)]
pub struct GameResult {
    judged_guesses: Vec<JudgedGuess>,
    state: State,
}

#[derive(Debug)]
pub struct JudgedGuess {
    guess: String,
    result: Vec<JudgedChar>,
}

#[derive(Debug)]
pub enum JudgedChar {
    Correct,
    WrongPlace,
    Wrong,
}

#[derive(Debug)]
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
