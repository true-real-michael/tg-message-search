use leptos::html::Input;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::web_sys::HtmlInputElement;

async fn print_file_content(input: Option<HtmlInputElement>) -> Option<String> {
    log!("Input: {:?}", input);
    let value = input.unwrap().files();
    log!("files: {:?}", value);
    let value_unwrapped = value.unwrap();
    let get_file = value_unwrapped.get(0);
    log!("File option: {:?}", get_file);
    let file_text = get_file.unwrap().text();
    // log!("File text: {:?}", file_text);
    let result = wasm_bindgen_futures::JsFuture::from(file_text).await;
    // log!("Result: {:?}", result);
    result.ok().map(|value| value.as_string().unwrap())
}

#[component]
pub fn FileInput(set_input_data: WriteSignal<Option<String>>) -> impl IntoView {
    let file_input: NodeRef<Input> = NodeRef::new();

    view! {
        <h3>File Upload</h3>
        <div>
            <input
                type="file"
                node_ref=file_input
                on:change=move |e| {
                    let file_input_value = file_input.get();
                    spawn_local(async move {
                        let content = print_file_content(file_input_value).await;
                        set_input_data.set(content);
                    })
                }
            />
        </div>
    }
}
