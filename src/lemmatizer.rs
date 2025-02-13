use std::collections::HashMap;

const DICTIONARY_RAW: &str = include_str!("../data/lemmatization-ru.tsv");

#[derive(Default)]
pub struct Lemmatizer {
    dict: HashMap<String, String>,
}

impl Lemmatizer {
    pub fn new() -> Self {
        let dict = DICTIONARY_RAW
            .split(|char| char == '\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut parts = line.splitn(2, |c| c == '\t');
                let word = parts.next().unwrap().to_string();
                let lemma = parts.next().unwrap().to_string();
                (word, lemma)
            })
            .collect();
        Self { dict }
    }

    pub fn lemmatize(&self, word: &str) -> String {
        self.dict
            .get(word)
            .cloned()
            .unwrap_or_else(|| word.to_string())
    }
}
