use crate::analysis::utils;
use flate2::read::GzDecoder;
use rayon::prelude::*;
use std::collections::HashMap;
use std::io::prelude::Read;

const DICTIONARY_RAW: &[u8] = include_bytes!("../../data/lemmatization-ru.tsv.gz");

#[derive(Default, Clone)]
pub struct Lemmatizer {
    dict: HashMap<&'static str, &'static str>,
}

impl Lemmatizer {
    pub fn new() -> Self {
        let mut decoder = GzDecoder::new(DICTIONARY_RAW);
        let mut data = String::new();
        decoder.read_to_string(&mut data).unwrap();

        let data = Box::leak(data.into_boxed_str());

        let time_start = chrono::Utc::now();
        let dict: HashMap<&str, &str> = data
            .par_split('\n')
            .filter(|line| !line.is_empty())
            .flat_map(|line| {
                let mut parts = line.split('\t');
                let lemma = parts.next().unwrap();
                parts.map(move |word| (word, lemma)).par_bridge()
            })
            .collect();
        utils::log!("Lemmatizer loaded in {:?}", chrono::Utc::now() - time_start);
        Self { dict }
    }

    pub fn lemmatize<'a>(&'a self, word: &'a str) -> &'a str {
        self.dict.get(word).unwrap_or(&word)
    }
}
