import {process_file} from "wasm-chat-analysis"

// load a file and output the result
const text_file_input = document.getElementById("file-input")
const output = document.getElementById("output")
text_file_input.addEventListener("change", async () => {
    const file = text_file_input.files[0]
    output.innerText = await process_file(await file.text())
})
