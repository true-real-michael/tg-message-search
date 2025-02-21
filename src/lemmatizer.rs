use flate2::read::GzDecoder;
use std::collections::HashMap;
use std::io::prelude::Read;

const DICTIONARY_RAW: &[u8] = include_bytes!("../data/lemmatization-ru.tsv.gz");

#[derive(Default)]
pub struct Lemmatizer {
    dict: HashMap<&'static str, &'static str>,
}

impl Lemmatizer {
    pub fn new() -> Self {
        let mut decoder = GzDecoder::new(DICTIONARY_RAW);
        let mut data = String::new();
        decoder.read_to_string(&mut data).unwrap();

        let data = Box::leak(data.into_boxed_str());

        let dict: HashMap<&str, &str> = data
            .split('\n')
            .filter(|line| !line.is_empty())
            .flat_map(|line| {
                let mut parts = line.split('\t');
                let lemma = parts.next().unwrap();
                parts.map(move |word| (word, lemma))
            })
            .collect();

        Self { dict }
    }

    pub fn lemmatize(&self, word: &str) -> String {
        self.dict.get(word).unwrap_or(&word).to_string()
    }
}
