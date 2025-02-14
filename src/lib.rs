mod lemmatizer;
mod thread_dsu;
mod utils;

pub use crate::lemmatizer::Lemmatizer;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into());
//     }
// }

#[derive(Clone)]
pub struct Message {
    pub text: String,
    pub reply_to: Option<usize>,
    pub date_text: String,
}

#[derive(Clone)]
#[wasm_bindgen]
pub struct ThreadSearchResult {
    pub thread_id: usize,
    pub score: u32,
    title_text: String,
    date_text: String
}

#[wasm_bindgen]
impl ThreadSearchResult {
    #[wasm_bindgen(getter)]
    pub fn title_text(&self) -> String {
        self.title_text.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn date_text(&self) -> String {
        self.date_text.clone()
    }
}

#[derive(Clone)]
#[wasm_bindgen]
pub struct MessageResult {
    pub message_id: usize,
    text: String,
    reply_to_text: Option<String>,
}

#[wasm_bindgen]
impl MessageResult {
    #[wasm_bindgen(getter)]
    pub fn text(&self) -> String {
        self.text.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn reply_to_text(&self) -> Option<String> {
        self.reply_to_text.clone()
    }
}

#[wasm_bindgen]
pub struct Searcher {
    messages: Vec<Message>,
    threads: Vec<Vec<usize>>,
    lemmatizer: Lemmatizer,
    thread_index: HashMap<String, Vec<usize>>,
}

#[wasm_bindgen]
impl Searcher {
    pub fn new() -> Searcher {
        utils::set_panic_hook();
        Searcher {
            messages: Vec::new(),
            threads: Vec::new(),
            lemmatizer: Lemmatizer::new(),
            thread_index: HashMap::new(),
        }
    }

    pub fn set_data(&mut self, messages_csv_input: &str) {
        let mut thread_dsu = thread_dsu::ThreadDSU::new();
        let mut messages = Vec::new();
        let mut normalized_ids = HashMap::new();
        let mut current_id = 0;

        let mut reader = csv::Reader::from_reader(messages_csv_input.as_bytes());
        for result in reader.records() {
            let record = result.as_ref().expect("Error reading CSV record");
            let id = record
                .get(0)
                .expect("No id found")
                .parse::<usize>()
                .unwrap();
            let date_text = record.get(1).expect("No date found")[..7].to_string();
            let text = record.get(2).expect("No message found").to_string();
            let reply_to = record
                .get(3)
                .and_then(|s| s.parse::<usize>().ok())
                .filter(|&id| id != 0)
                .map(|id| normalized_ids.get(&id).copied())
                .flatten();
            messages.push(Message { text, reply_to, date_text });

            thread_dsu.make_set(current_id);
            normalized_ids.insert(id, current_id);
            current_id += 1;
        }

        for (id, message) in messages.iter().enumerate() {
            if let Some(reply_to) = message.reply_to {
                thread_dsu.union_sets(reply_to, id); // order is important
            }
        }

        let threads = thread_dsu.get_threads();

        let mut thread_index = HashMap::new();
        for (thread_id, message_ids) in threads.iter().enumerate() {
            for message_id in message_ids {
                messages[*message_id]
                    .text
                    .to_lowercase()
                    .replace(|c: char| !c.is_alphanumeric(), " ")
                    .split_whitespace()
                    .filter(|word| word.len() > 3)
                    .map(|word| self.lemmatizer.lemmatize(word))
                    .for_each(|word| {
                        thread_index
                            .entry(word)
                            .or_insert_with(Vec::new)
                            .push(thread_id);
                    });
            }
        }

        self.thread_index = thread_index;
        self.messages = messages;
        self.threads = threads;
    }

    pub fn find_threads(&self, query: &str, sort_by: u8) -> Vec<ThreadSearchResult> {
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
                    *thread_scores.entry(*thread).or_insert(0) += 1;
                }
            }
        }

        let mut thread_search_results = thread_scores
            .iter()
            .map(|(&thread_id, &score)| ThreadSearchResult {
                thread_id,
                score,
                title_text: self.messages[self.threads[thread_id][0]].text.clone(),
                date_text: self.messages[self.threads[thread_id][0]].date_text.clone(),
            })
            .collect::<Vec<_>>();
        if sort_by == 0 { // sort by thread_id
            thread_search_results.sort_by_key(|result| -(result.thread_id as i32));
        } else { // sort by score and then by thread_id
            thread_search_results.sort_by_key(|result| (-(result.score as i32), -(result.thread_id as i32)));
        }

        thread_search_results
    }

    pub fn get_thread_messages(&self, thread_id: usize) -> Vec<MessageResult> {
        let min_id = self.threads[thread_id].first().copied().unwrap();
        let max_id = self.threads[thread_id].last().copied().unwrap();
        self.get_message_range(min_id, max_id)
    }

    pub fn get_message_range(&self, message_id_min: usize, message_id_max: usize) -> Vec<MessageResult> {
        self.messages[message_id_min..=message_id_max].iter().map(|message| {
            let reply_to_text = message
                .reply_to
                .map(|reply_to_id| self.messages[reply_to_id].text.clone());
            MessageResult {
                message_id: message_id_min,
                text: message.text.clone(),
                reply_to_text,
            }
        }).collect()
    }
}
