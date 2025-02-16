mod deserialization;
mod lemmatizer;
mod thread_dsu;
mod utils;

pub use crate::lemmatizer::Lemmatizer;
use deserialization::{deserialize_messages, Message};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(Clone)]
#[wasm_bindgen]
pub struct ThreadSearchResult {
    pub thread_id: u32,
    pub score: u32,
    title_text: String,
    pub date_unixtime: u32,
}

#[wasm_bindgen]
impl ThreadSearchResult {
    #[wasm_bindgen(getter)]
    pub fn title_text(&self) -> String {
        self.title_text.clone()
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

impl Default for Searcher {
    fn default() -> Self {
        utils::set_panic_hook();
        Searcher {
            messages: Vec::new(),
            threads: Vec::new(),
            lemmatizer: Lemmatizer::new(),
            thread_index: HashMap::new(),
        }
    }
}

#[wasm_bindgen]
impl Searcher {
    pub fn new() -> Searcher {
        Self::default()
    }

    pub fn set_data(&mut self, json: &str) -> Result<(), JsError> {
        let mut thread_dsu = thread_dsu::ThreadDSU::new();
        let messages = deserialize_messages(json).map_err(|e| JsError::from(&*e))?;

        for message in &messages {
            thread_dsu.make_set(message.id);
        }

        for (id, message) in messages.iter().enumerate() {
            if let Some(reply_to) = message.reply_to_message_id {
                thread_dsu.union_sets(reply_to, id); // order is important
            }
        }

        let threads = thread_dsu.get_threads();

        let mut thread_index = HashMap::new();
        for (thread_id, message_ids) in threads.iter().enumerate() {
            for message_id in message_ids {
                for text_entity in &messages[*message_id].text_entities {
                    if let deserialization::TextEntity::Lemmatizable(text) = text_entity {
                        text.to_lowercase()
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
            }
        }

        self.thread_index = thread_index;
        self.messages = messages;
        self.threads = threads;
        Ok(())
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
                thread_id: thread_id as u32,
                score,
                title_text: self.messages[self.threads[thread_id][0]].clone().into(),
                date_unixtime: self.messages[self.threads[thread_id][0]].date_unixtime,
            })
            .collect::<Vec<_>>();
        if sort_by == 0 {
            // sort by thread_id
            thread_search_results.sort_by_key(|result| -(result.date_unixtime as i32));
        } else {
            // sort by score and then by thread_id
            thread_search_results
                .sort_by_key(|result| (-(result.score as i32), -(result.date_unixtime as i32)));
        }

        thread_search_results
    }

    pub fn get_thread_messages(&self, thread_id: usize) -> Vec<MessageResult> {
        utils::log!("get_thread_messages({})", thread_id);
        let min_id = self.threads[thread_id].first().copied().unwrap();
        let max_id = self.threads[thread_id].last().copied().unwrap();
        self.get_message_range(min_id, max_id)
    }

    pub fn get_message_range(
        &self,
        message_id_min: usize,
        message_id_max: usize,
    ) -> Vec<MessageResult> {
        utils::log!("get_message_range({}, {})", message_id_min, message_id_max);
        if message_id_min > message_id_max {
            return Vec::new();
        }
        self.messages[message_id_min..=message_id_max]
            .iter()
            .map(|message| {
                let reply_to_text = message
                    .reply_to_message_id
                    .map(|reply_to_id| self.messages[reply_to_id].clone().into());
                MessageResult {
                    message_id: message.id,
                    text: message.clone().into(),
                    reply_to_text,
                }
            })
            .collect()
    }
}
