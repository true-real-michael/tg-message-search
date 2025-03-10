use leptos::logging::log;
use std::sync::{Arc, Mutex};

use crate::analysis::Searcher;
use crate::analysis::{MessageResult, ThreadSearchResult};
use chrono::DateTime;
use leptos::html;
use leptos::prelude::*;

#[component]
pub fn Search(searcher: LocalResource<Option<Arc<Mutex<Searcher>>>>) -> impl IntoView {
    let (search_query, set_search_query) = signal(String::new());
    let (selected_thread_id, set_selected_thread_id) = signal(None::<u32>);

    let result_threads = Memo::new(move |_| {
        log!("Searching for threads...");
        if let Some(searcher) = searcher.read().as_deref() {
            searcher
                .as_ref()
                .unwrap()
                .lock()
                .unwrap()
                .find_threads(search_query.get().clone())
                .unwrap_or_default()
        } else {
            Vec::new()
        }
    });

    let result_messages = Memo::new(move |_| {
        log!("Retrieving messages...");
        let selected_thread_id = selected_thread_id.get();
        if let Some(selected_thread_id) = selected_thread_id {
            if let Some(searcher) = searcher.read().as_deref() {
                searcher
                    .as_ref()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .get_thread_messages(selected_thread_id as usize)
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    });

    view! {
        <SearchBar set_search_query=set_search_query />
        <div class="grid grid-cols-2 gap-8 h-[calc(100vh-102px)]">
            <div class="overflow-y-auto">
                <ThreadList
                    threads=result_threads
                    set_selected_thread_id=set_selected_thread_id
                />
            </div>
            <div class="overflow-y-auto">
                <MessageList messages=result_messages />
            </div>
        </div>
    }
}

#[component]
pub fn ThreadList(
    threads: Memo<Vec<ThreadSearchResult>>,
    set_selected_thread_id: WriteSignal<Option<u32>>,
) -> impl IntoView {
    view! {
        <ul>
            {move || {
                threads.with(|threads| {
                    threads.clone().into_iter().map(|thread| {
                        let date = DateTime::from_timestamp(thread.date_unixtime as i64, 0).expect("Failed to parse date").format("%Y-%m").to_string();
                        view! {
                            <li class="p-2 hover:bg-gray-700 cursor-pointer" data-id={thread.thread_id} on:click=move |_| set_selected_thread_id.set(Some(thread.thread_id))>
                                <div class="flex justify-between items-center">
                                    <span class="truncate">{thread.title_text.clone()}</span>
                                    <span class="text-sm ml-2 whitespace-nowrap">{date}</span>
                                </div>
                            </li>
                        }
                    }).collect::<Vec<_>>()
                })
            }}
        </ul>
    }
}

#[component]
pub fn MessageList(messages: Memo<Vec<MessageResult>>) -> impl IntoView {
    view! {
        <ul>
            {move || {
                messages.with(|messages| {
                    messages.clone().into_iter().map(|message| {
                        let reply_text = message.reply_to_text.clone();
                        let message_text = message.text.clone();
                        view! {
                            <li class="p-2">
                                <div class="bg-sky-400/25 border-sky-700/40 border rounded p-2 m-2">
                                    <Show when=move || {message.reply_to_text.is_some()}>
                                        <div class="truncate ml-2 bg-gray-900/40 rounded p-1">
                                            {reply_text.clone()}
                                        </div>
                                    </Show>
                                    {message_text.clone()}
                                </div>
                            </li>
                        }
                    }).collect::<Vec<_>>()
                })
            }}
        </ul>
    }
}

#[component]
pub fn SearchBar(set_search_query: WriteSignal<String>) -> impl IntoView {
    let input_element: NodeRef<html::Input> = NodeRef::new();
    view! {
            <form on:submit= move |e| {
                e.prevent_default();
                let value = input_element.get().expect("input element").value();
                set_search_query.set(value);
            }>
                <div class="mb-6 flex">
                        <input type="text" placeholder="search" class="w-full bg-gray-700 p-2 border border-gray-300 rounded" node_ref=input_element />
                        <input type="submit" value="Search" class="ml-2 p-2 border bg-sky-400/25 border-sky-600 rounded" />
                </div>
            </form>
    }
}
