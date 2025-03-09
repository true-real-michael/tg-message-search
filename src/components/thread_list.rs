use leptos::prelude::*;

use crate::analysis::{MessageResult, ThreadSearchResult};

#[component]
pub fn ThreadList(
    threads: Vec<ThreadSearchResult>,
    set_selected_thread_id: WriteSignal<Option<u32>>,
) -> impl IntoView {
    view! {
        <ul>
            {move || {
                threads.clone().into_iter().map(|thread| {
                    view! {
                        <li class="p-2 hover:bg-gray-700 cursor-pointer" data-id={thread.thread_id} on:click=move |_| set_selected_thread_id.set(Some(thread.thread_id))>
                            <div class="flex justify-between items-center">
                                <span class="truncate">{thread.title_text.clone()}</span>
                                <span class="text-sm ml-2 whitespace-nowrap">{thread.date_unixtime}</span>
                            </div>
                        </li>
                    }
                }).collect::<Vec<_>>()
            }}
        </ul>
    }
}

#[component]
pub fn MessageList(
    messages: Vec<MessageResult>,
) -> impl IntoView {
    view! {
        <ul>
            {move || {
                messages.clone().into_iter().map(|message| {
                    view! {
                        <li class="p-2">
                            <div class="flex justify-between items-center">
                                <span class="truncate">{message.text.clone()}</span>
                            </div>
                        </li>
                    }
                }).collect::<Vec<_>>()
            }}
        </ul>
    }
}