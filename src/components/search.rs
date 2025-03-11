use leptos::either::Either;
use leptos::logging::log;
use std::sync::{Arc, Mutex};
use web_sys::MouseEvent;

use crate::analysis::{MessageResult, ThreadSearchResult};
use crate::analysis::{Searcher, Text};
use chrono::DateTime;
use leptos::html;
use leptos::prelude::*;

#[component]
pub fn Search(searcher: LocalResource<Option<Arc<Mutex<Searcher>>>>) -> impl IntoView {
    let (search_query, set_search_query) = signal(String::new());
    let (selected_thread_id, set_selected_thread_id) = signal(None::<u32>);
    let (offset_before, set_offset_before) = signal(0usize);
    let (offset_after, set_offset_after) = signal(0usize);

    let query_words = Memo::new(move |_| {
        log!("getting query words...");
        if let Some(searcher) = searcher.read().as_deref() {
            searcher
                .as_ref()
                .unwrap()
                .lock()
                .unwrap()
                .get_query_words(search_query.get().clone())
        } else {
            Vec::new()
        }
    });

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

    let message_border_ids = Memo::new(move |_| {
        log!("Retrieving messages...");
        let selected_thread_id = selected_thread_id.get();
        if let Some(selected_thread_id) = selected_thread_id {
            if let Some(searcher) = searcher.read().as_deref() {
                set_offset_after.set(0);
                set_offset_before.set(0);
                let (min_id, max_id) = searcher
                    .as_ref()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .get_thread_messages(selected_thread_id as usize);
                Some((min_id, max_id))
            } else {
                None
            }
        } else {
            None
        }
    });

    let messages = Memo::new(move |_| {
        if let Some((min_id, max_id)) = message_border_ids.get() {
            if let Some(searcher) = searcher.read().as_deref() {
                log!(
                    "Retrieving messages..., min_id: {}, max_id: {}",
                    min_id,
                    max_id
                );
                let query_words = query_words.get().clone();
                searcher
                    .as_ref()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .get_message_range(
                        min_id.saturating_sub(offset_before.get()),
                        max_id.saturating_add(offset_after.get()),
                        query_words,
                    )
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
                <MessageList messages=messages set_offset_before=set_offset_before set_offset_after=set_offset_after />
            </div>
        </div>
    }
}

#[component]
fn ThreadList(
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
                                    <span class="text-sm whitespace-nowrap">{date}</span>
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
fn MessageList(
    messages: Memo<Vec<MessageResult>>,
    set_offset_before: WriteSignal<usize>,
    set_offset_after: WriteSignal<usize>,
) -> impl IntoView {
    view! {
        <ul>
        {move || {
            messages.with(|messages| {
                let btn_before = view! {
                    <li class="p-2"><Button on_click=move |_| *set_offset_before.write() += 5 /></li>
                };
                    let view_messages = messages.clone().into_iter().map(|message| {
                        let reply_text = message.reply_to_text.clone().map(|text| {
                            view! {
                                <div class="truncate bg-gray-900/40 rounded p-1">
                                    {text}
                                </div>
                            }
                        });
                        let message_text = message.text.clone();
                        let highlighted_text = message_text.into_iter().map(|text| {
                            let text = text.clone();
                            match text {
                                Text::Highlight(text) => {
                                    Either::Left(view! {
                                        <span class="bg-yellow-400/50 rounded p-1">{text.clone()}</span>
                                    })
                                },
                                Text::Plain(text) => {
                                    Either::Right(view! {
                                        <span>{text.clone()}</span>
                                    })
                                }
                            }
                        }).collect::<Vec<_>>();
                        view! {
                            <li class="p-2">
                                <div class="bg-sky-400/25 border-sky-700/40 border rounded p-2">
                                    {reply_text.clone()}
                                    {highlighted_text}
                                </div>
                            </li>
                        }
                    }).collect::<Vec<_>>();

                    let btn_after = view! {
                        <li class="p-2"><Button on_click=move |_| *set_offset_after.write() += 5 /></li>
                    };

                    (btn_before, view_messages, btn_after)
                })
            }}
        </ul>
    }
}

#[component]
fn SearchBar(set_search_query: WriteSignal<String>) -> impl IntoView {
    let input_element: NodeRef<html::Input> = NodeRef::new();
    view! {
            <form on:submit= move |e| {
                e.prevent_default();
                let value = input_element.get().expect("input element").value();
                set_search_query.set(value);
            }>
                <div class="mb-6 flex">
                        <input type="text" placeholder="search" class="w-full bg-gray-700 p-2 border border-gray-300 rounded" node_ref=input_element />
                        <input type="submit" value="Search" class="ml-2 p-2 border bg-sky-400/25 border-sky-600 rounded hover:bg-sky-400/50 transition-colors cursor-pointer" />
                </div>
            </form>
    }
}
#[component]
fn Button(on_click: impl FnMut(MouseEvent) + 'static) -> impl IntoView {
    view! {
        <button on:click=on_click class="w-full p-2 bg-sky-400/25 border-sky-600 border rounded hover:bg-sky-400/50 transition-colors">
            { "Load More" }
        </button>
    }
}
