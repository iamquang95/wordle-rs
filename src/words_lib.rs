use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

use anyhow::Result;

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
                words.insert(word);
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
}
