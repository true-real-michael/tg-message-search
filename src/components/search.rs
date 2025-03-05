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
            <input type="text" value="search" node_ref=input_element />
            <input type="submit" value="Search" />
        </form>
        <div>
            <h1>"Search"</h1>
            <p>"This is the search page"</p>
        </div>
        <div>
            <h2>"Results"</h2>
            <ul>
                <li>"Result 1"</li>
                <li>"Result 2"</li>
                <li>"Result 3"</li>
            </ul>
        </div>
    }
}
