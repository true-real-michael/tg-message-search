use leptos::either::Either;
use leptos::logging::log;
use leptos::prelude::*;

use crate::analysis::{Lemmatizer, ThreadSearchResult};
use crate::components::file_input::FileInput;
use crate::components::thread_list::{MessageList, ThreadList};
use crate::{analysis::Searcher, components::search::Search};
use std::sync::{Arc, Mutex};

async fn load_searcher(
    lemmatizer: Arc<Mutex<Lemmatizer>>,
    input_data: Option<String>,
) -> Option<Arc<Mutex<Searcher>>> {
    log!("Initializing searcher...");
    if let Some(input_data) = input_data {
        let lemmatizer = lemmatizer.clone();
        Some(Arc::new(Mutex::new(
            Searcher::new(lemmatizer, input_data).ok()?,
        )))
    } else {
        None
    }
}

async fn load_result_threads(
    searcher: Option<Arc<Mutex<Searcher>>>,
    search_query: String,
) -> Vec<ThreadSearchResult> {
    log!("Searching for threads...");
    if let Some(searcher) = searcher {
        searcher
            .lock()
            .unwrap()
            .find_threads(search_query)
            .unwrap_or_default()
    } else {
        Vec::new()
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let (messages_json, set_messages_json) = signal(None::<String>);
    let (search_query, set_search_query) = signal(String::new());
    let (selected_thread_id, set_selected_thread_id) = signal(None::<u32>);
    let lemmatizer = Arc::new(Mutex::new(Lemmatizer::default()));

    let searcher = LocalResource::new(move || {
        let lemmatizer = lemmatizer.clone();
        let messages_json = messages_json.get().clone();
        async move { load_searcher(lemmatizer, messages_json).await }
    });

    let result_threads = LocalResource::new(move || {
        let searcher = searcher.read().as_deref().flatten().cloned();
        let search_query = search_query.get().clone();
        async move { load_result_threads(searcher, search_query).await }
    });

    let result_messages = LocalResource::new(move || {
        let searcher = searcher.read().as_deref().flatten().cloned();
        let selected_thread_id = selected_thread_id.get().clone();
        async move {
            if let Some(selected_thread_id) = selected_thread_id {
                searcher
                    .map(|s| s.lock().unwrap().get_thread_messages(selected_thread_id as usize))
                    .into_iter()
                    .flatten()
                    .collect()
            } else {
                Vec::new()
            }
        }
    });

    view! {
        <div class="bg-gray-900/40 container mx-auto p-4">
            {move || {
                if searcher.read().as_ref().map_or(true, |s| s.is_none()) {
                    Either::Left(view! {
                        <FileInput set_input_data=set_messages_json />
                    })
                } else {
                    Either::Right(view! {
                        <Search set_search_query=set_search_query.clone() />

                        <div class="grid grid-cols-2 gap-8 h-[calc(100vh-102px)]">
                            <ThreadList threads=result_threads.get().map_or_else(Vec::default, |t| t.to_vec()) set_selected_thread_id=set_selected_thread_id />
                            <MessageList messages=result_messages.get().map_or_else(Vec::default, |m| m.to_vec()) />
                        </div>
                    })
                }
            }}
        </div>
    }
}
