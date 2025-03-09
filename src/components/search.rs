use crate::analysis::{MessageResult, ThreadSearchResult};
use leptos::html;
use leptos::prelude::*;

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
pub fn MessageList(messages: Vec<MessageResult>) -> impl IntoView {
    view! {
        <ul>
            {move || {
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
            }}
        </ul>
    }
}

#[component]
pub fn Search(set_search_query: WriteSignal<String>) -> impl IntoView {
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
