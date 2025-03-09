use leptos::either::Either;
use leptos::logging::log;
use leptos::prelude::*;

use crate::analysis::{Searcher, Lemmatizer};
use crate::components::file_input::FileInput;
use crate::components::search::Search;
use std::sync::{Arc, Mutex};

async fn load_searcher(
    lemmatizer: Arc<Mutex<Lemmatizer>>,
    input_data: Option<String>,
) -> Option<Arc<Mutex<Searcher>>> {
    log!("Initializing searcher...");
    let lemmatizer = lemmatizer.clone();
    Some(Arc::new(Mutex::new(
        Searcher::new(lemmatizer, input_data?).ok()?,
    )))
}

#[component]
pub fn Home() -> impl IntoView {
    let (messages_json, set_messages_json) = signal(None::<String>);
    let lemmatizer = Arc::new(Mutex::new(Lemmatizer::new()));

    let searcher = LocalResource::new(move || {
        let lemmatizer = lemmatizer.clone();
        let messages_json = messages_json.get().clone();
        async move { load_searcher(lemmatizer, messages_json).await }
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
                        <Show when=move || {
                            searcher.read().as_deref().flatten().cloned().is_some()
                        }>
                            <Search searcher=searcher />
                        </Show>
                    })
                }
            }}
        </div>
    }
}
