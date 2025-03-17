use leptos::either::Either;
use leptos::logging::log;
use leptos::prelude::*;

use crate::analysis::{Lemmatizer, Searcher};
use crate::components::file_input::FileInput;
use crate::components::search::Search;
use std::sync::{Arc, Mutex};

#[component]
pub fn Home() -> impl IntoView {
    let (messages_json, set_messages_json) = signal(None::<String>);
    let lemmatizer = Box::leak(Box::new(Lemmatizer::new()));
    let lemmatizer = &*lemmatizer;

    let searcher = LocalResource::new(move || {
        let messages_json = messages_json.get().clone();
        async move {
            log!("Initializing searcher...");
            Some(Arc::new(Mutex::new(
                Searcher::new(lemmatizer, messages_json?).ok()?,
            )))
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
