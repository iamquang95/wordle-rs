use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{anyhow, Result};
use rand::Rng;

pub struct WordsLib {
    words: HashSet<String>,
    popular_words: HashSet<String>,
}

impl WordsLib {
    pub fn load(full_words: &str, popular_words: &str) -> Result<WordsLib> {
        let words1 = WordsLib::load_words(full_words)?;
        let words2 = WordsLib::load_words(popular_words)?;
        let words = words1.union(&words2).map(|word| word.to_owned()).collect();
        Ok(WordsLib {
            words,
            popular_words: words2,
        })
    }

    fn load_words(path: &str) -> Result<HashSet<String>> {
        let file = File::open(path)?;
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
        Ok(words)
    }

    pub fn contains(&self, word: &str) -> bool {
        self.words.contains(word)
    }

    pub fn random_popular_word(&self) -> Result<String> {
        let mut rng = rand::thread_rng();
        let word_idx = rng.gen_range(0..self.popular_words.len());
        let x = self
            .popular_words
            .iter()
            .nth(word_idx)
            .ok_or(anyhow!("Failed to random word"))?;
        Ok(x.clone())
    }
}
