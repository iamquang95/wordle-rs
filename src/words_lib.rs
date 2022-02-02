use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

use anyhow::{anyhow, Result};
use rand::Rng;

pub struct WordsLib {
    words: HashSet<String>,
}

impl WordsLib {
    pub fn load(file_path: &str) -> Result<WordsLib> {
        let mut file = File::open(file_path)?;
        let mut reader = BufReader::new(file);
        let mut words = HashSet::<String>::new();
        loop {
            let mut word = String::new();
            let read_result = reader.read_line(&mut word)?;
            if read_result == 0 {
                break;
            }
            if word.len() > 0 {
                words.insert(word.trim().to_ascii_uppercase());
            }
        }
        Ok(WordsLib { words })
    }

    pub fn merge(&self, another: &WordsLib) -> WordsLib {
        let words: HashSet<String> = self
            .words
            .union(&another.words)
            .map(|word| word.to_owned())
            .collect();
        WordsLib { words }
    }

    pub fn contains(&self, word: &str) -> bool {
        self.words.contains(word)
    }

    pub fn words_size(&self) -> usize {
        self.words.len()
    }

    pub fn random_word(&self) -> Result<String> {
        let mut rng = rand::thread_rng();
        let word_idx = rng.gen_range(0..self.words_size());
        let x = self
            .words
            .iter()
            .nth(word_idx)
            .ok_or(anyhow!("Failed to random word"))?;
        Ok(x.clone())
    }
}
