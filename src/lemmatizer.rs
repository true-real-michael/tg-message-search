use flate2::read::GzDecoder;
use std::collections::HashMap;
use std::io::prelude::Read;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

const DICTIONARY_RAW: &[u8] = include_bytes!("../data/lemmatization-ru.tsv.gz");

#[derive(Default)]
pub struct Lemmatizer {
    dict: HashMap<String, String>,
}

impl Lemmatizer {
    pub fn new() -> Self {
        log!("file length: {:?}", DICTIONARY_RAW.len());
        let mut decoder = GzDecoder::new(DICTIONARY_RAW);
        let mut data = String::new();
        decoder.read_to_string(&mut data).unwrap();
        let dict: HashMap<String, String> = data
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
