use leptos::prelude::*;

use crate::analysis::ThreadSearchResult;

#[component]
pub fn ThreadList(threads: Vec<ThreadSearchResult>) -> impl IntoView {
    view! {
        <ul>
            || move {threads.iter().map(|thread| {
                view! {
                    <li>
                        <a href=format!("/thread/{}", thread.thread_id)>
                            {thread.title_text.clone()}
                        </a>
                    </li>
                }
            }).collect::<Vec<_>>() }
        </ul>
    }
}

#[component]
fn ThreadListItem(thread: ThreadSearchResult) -> impl IntoView {
    view! {
        <li>
            <a href=format!("/thread/{}", thread.thread_id)>
                {thread.title_text}
            </a>
        </li>
    }
}
