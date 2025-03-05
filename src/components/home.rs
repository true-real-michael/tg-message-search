use leptos::logging::log;
use leptos::prelude::*;

use crate::analysis::{Lemmatizer, ThreadSearchResult};
use crate::components::file_input::FileInput;
use crate::components::thread_list::ThreadList;
use crate::{analysis::Searcher, components::search::Search};
use std::sync::{Arc, Mutex};

async fn load_searcher(
    lemmatizer: Arc<Mutex<Lemmatizer>>,
    input_data: String,
) -> Option<Arc<Mutex<Searcher>>> {
    log!("Initializing searcher...");
    Searcher::new(lemmatizer, input_data)
        .map(|searcher| Arc::new(Mutex::new(searcher)))
        .ok()
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
    let lemmatizer = Arc::new(Mutex::new(Lemmatizer::new()));

    let searcher = LocalResource::new(move || {
        let lemmatizer = lemmatizer.clone();
        let messages_json = messages_json.get().unwrap_or_default();
        async move { load_searcher(lemmatizer, messages_json).await }
    });

    let result_threads = LocalResource::new(move || {
        let searcher = searcher.read().as_deref().flatten().cloned();
        let search_query = search_query.get().clone();
        async move { load_result_threads(searcher, search_query).await }
    });

    view! {
        <FileInput set_input_data=set_messages_json />
        <Suspense fallback=move || view! { <p>Loading...</p> }>
            {move || {
                searcher.read().as_ref().map(|searcher| {
                    view! {
                        <Search set_search_query=set_search_query.clone() />
                    }
                })
            }}
        </Suspense>

        <Suspense fallback=move || view! { <p>Loading...</p> }>
            {move || {
                result_threads.read().as_ref().map(|result_threads| {
                    view! {
                        <ThreadList threads=result_threads.to_vec() />
                    }
                })
            }}
        </Suspense>


    }
}
