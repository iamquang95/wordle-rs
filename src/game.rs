use crate::WORDS_LIB;
use std::collections::HashSet;

use anyhow::{anyhow, Result};

pub struct Game {
    word: String,
    pub num_guesses: usize,
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

    pub fn word_len(&self) -> usize {
        self.word.len()
    }

    pub fn turn(&self) -> usize {
        self.state.guesses.len()
    }

    pub fn current_result(&self) -> GameResult {
        let existing_chars = self.word.clone().chars().collect::<HashSet<char>>();
        let judged_guesses = self
            .state
            .guesses
            .iter()
            .map(|guess| {
                let result = guess
                    .chars()
                    .zip(self.word.chars())
                    .map(|(lhs, rhs)| {
                        if lhs == rhs {
                            JudgedChar::Correct
                        } else if existing_chars.contains(&lhs) {
                            JudgedChar::WrongPlace
                        } else {
                            JudgedChar::Wrong
                        }
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
    pub judged_guesses: Vec<JudgedGuess>,
    pub state: State,
}

#[derive(Debug, Clone)]
pub struct JudgedGuess {
    pub guess: String,
    pub result: Vec<JudgedChar>,
}

#[derive(Debug, Clone)]
pub enum JudgedChar {
    Correct,
    WrongPlace,
    Wrong,
}

#[derive(Debug, Clone)]
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
