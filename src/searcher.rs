use crate::deserialization::{deserialize_messages, Message, TextEntity};
use crate::lemmatizer::Lemmatizer;
use crate::merge::{MergeAnd, MergeOr};
use crate::query::{Lexer, Parser, SearchQuery};
use crate::thread_dsu::ThreadDSU;
use crate::utils;
use std::collections::{HashMap, HashSet};
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
        let mut thread_dsu = ThreadDSU::new();
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
            let mut used_words = HashSet::new();
            for message_id in message_ids {
                for text_entity in &messages[*message_id].text_entities {
                    if let TextEntity::Lemmatizable(text) = text_entity {
                        text.to_lowercase()
                            .split(|c: char| !c.is_alphanumeric())
                            .filter(|word| word.len() > 3)
                            .map(|word| self.lemmatizer.lemmatize(word))
                            .for_each(|word| {
                                if !used_words.contains(&word) {
                                    thread_index
                                        .entry(word.clone())
                                        .or_insert_with(Vec::new)
                                        .push(thread_id);
                                    used_words.insert(word);
                                }
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

    fn find_threads_by_word(&self, word: String) -> Vec<usize> {
        utils::log!("find_threads_by_word({})", word);
        let word = word.to_lowercase();
        self.thread_index.get(&word).cloned().unwrap_or_default()
    }

    fn find_threads_by_query(&self, query: SearchQuery) -> Vec<usize> {
        match query {
            SearchQuery::Word(word) => self.find_threads_by_word(word),
            SearchQuery::Or((query_left, query_right)) => MergeOr::new(
                self.find_threads_by_query(*query_left).iter(),
                self.find_threads_by_query(*query_right).iter(),
            )
            .copied()
            .collect(),
            SearchQuery::And((query_left, query_right)) => MergeAnd::new(
                self.find_threads_by_query(*query_left).iter(),
                self.find_threads_by_query(*query_right).iter(),
            )
            .copied()
            .collect(),
        }
    }

    pub fn find_threads(&self, query: &str) -> Result<Vec<ThreadSearchResult>, JsError> {
        let query = Parser::new(Lexer::new(query))
            .map_err(|e| JsError::from(&*e))?
            .parse()
            .map_err(|e| JsError::from(&*e))?;

        let mut result: Vec<ThreadSearchResult> = self
            .find_threads_by_query(query)
            .into_iter()
            .map(|thread_id| {
                let message_id = self.threads[thread_id].first().copied().unwrap();
                let message = &self.messages[message_id];
                ThreadSearchResult {
                    thread_id: thread_id as u32,
                    score: 0,
                    title_text: message.clone().into(),
                    date_unixtime: message.date_unixtime,
                }
            })
            .collect();
        result.sort_by_key(|thread| -(thread.date_unixtime as i32));
        Ok(result)
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
