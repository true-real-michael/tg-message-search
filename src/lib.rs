mod lemmatizer;
mod thread_dsu;
mod utils;

use crate::lemmatizer::Lemmatizer;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive!(Clone)]
pub struct Message {
    pub text: String,
    pub reply_to: Option<usize>,
}

#[derive!(Clone)]
pub struct ThreadSearchResult {
    pub thread_id: usize,
    pub score: u32,
    pub title_text: String,
}

#[derive!(Clone)]
pub struct MessageResult {
    pub message_id: usize,
    pub text: String,
    pub reply_to_text: Option<String>,
}

pub struct Searcher {
    messages: Vec<Message>,
    threads: Vec<Vec<usize>>,
    lemmatizer: Lemmatizer,
    thread_index: HashMap<String, Vec<usize>>,
}

#[wasm_bindgen]
impl Searcher {
    pub fn new(messages_csv_input: &str) -> Searcher {
        let mut thread_dsu = thread_dsu::ThreadDSU::new();
        let mut messages = Vec::new();
        let mut normalized_ids = HashMap::new();
        let mut current_id = 0;

        for line in messages_csv_input.lines() {
            let mut parts = line.split(',');
            let id = parts.next().unwrap().parse::<u32>().unwrap();
            let text = parts.next().unwrap().to_string();
            let reply_to = parts
                .next()
                .map(|s| s.parse::<u32>().unwrap())
                .map(|id| normalized_ids[&id]);

            let message = Message { text, reply_to };
            messages.push(message);

            thread_dsu.make_set(current_id);
            normalized_ids.insert(id, current_id);
            current_id += 1;
        }

        for (id, message) in messages.iter().enumerate() {
            if let Some(reply_to) = message.reply_to {
                thread_dsu.union_sets(id, reply_to);
            }
        }

        let threads = thread_dsu.get_threads();
        let lemmatizer = Lemmatizer::new();
        let message_thread_map = thread_dsu.get_reverse_mapping();

        let mut thread_index = HashMap::new();
        for (message_id, thread_id) in message_thread_map {
            messages[message_id]
                .text
                .to_lowercase()
                .replace(|c: char| !c.is_alphanumeric(), " ")
                .split_whitespace()
                .filter(|word| word.len() > 3)
                .map(|word| lemmatizer.lemmatize(word))
                .for_each(|word| {
                    thread_index
                        .entry(word)
                        .or_insert_with(Vec::new)
                        .push(thread_id);
                });
        }

        Searcher {
            messages,
            threads,
            lemmatizer,
            thread_index,
        }
    }

    pub fn find_threads(&self, query: &str) -> Vec<ThreadSearchResult> {
        let query = query
            .to_lowercase()
            .replace(|c: char| !c.is_alphanumeric(), " ")
            .split_whitespace()
            .filter(|word| word.len() > 3)
            .map(|word| self.lemmatizer.lemmatize(word))
            .collect::<Vec<_>>();

        let mut thread_scores = HashMap::new();
        for word in &query {
            if let Some(threads) = self.thread_index.get(word) {
                for thread in threads {
                    *thread_scores.entry(thread).or_insert(0) += 1;
                }
            }
        }

        let mut thread_search_results = thread_scores
            .iter()
            .map(|(&&thread_id, &score)| ThreadSearchResult {
                thread_id,
                score,
                title_text: self.messages[self.threads[thread_id][0]].text.clone(),
            })
            .collect::<Vec<_>>();

        thread_search_results.sort_by_key(|result| result.score);

        thread_search_results
    }

    pub fn get_thread_messages(&self, thread_id: usize) -> Vec<MessageResult> {
        self.threads[thread_id]
            .iter()
            .map(|&message_id| {
                let message = &self.messages[message_id];
                let reply_to_text = message
                    .reply_to
                    .map(|reply_to_id| self.messages[reply_to_id].text.clone());
                MessageResult {
                    message_id,
                    text: message.text.clone(),
                    reply_to_text,
                }
            })
            .collect()
    }
}

