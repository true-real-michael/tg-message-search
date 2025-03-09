use leptos::html;
use leptos::prelude::*;

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
