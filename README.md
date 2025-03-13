# Telegram Message Search
## Usage

The app is currenty hosted here: [https://true-real-michael.github.io/tg-message-search](https://true-real-michael.github.io/tg-message-search)

1. [Export](https://telegram.org/blog/export-and-more) a telegram **chat** in a JSON format
2. Upload the `result.json` file (the website runs in the browser and no data is saved)
3. Search and browse threads and messages

---

1. [Экспортируйте](https://telegram.org/blog/export-and-more) телеграм-чат в формате JSON
2. Загрузите файл `result.json` (сайт работает полностью в браузере, и никакие данные не сохраняются)
3. Производите поиск по тредам и сообщениям


UI looks like this:
![ui example](assets/example.jpg)

![wasm offer](assets/wasm-offer.jpg)


## Building

1. Install Trunk, add wasm32 target
```sh
cargo install trunk
rustup target add wasm32-unknown-unknown
```
2. Download the `lemmatization-ru.tsv.gz` file from releases and place it under the `/data` directory. Alternatively, download the morphological dictionary from [OpenCorpora's website](https://opencorpora.org/?page=downloads), place it under `/data`, run the `scripts/preprocess_opcorpora.py` script, and gzip the result
3. Run the project
```sh
trunk serve --port 3000 --release
```
4. The project will be available at localhost:3000/tg-message-search


## Why?

- The Telegram native message search was not convenient for me, especially:
  - When I wanted to search for synonyms.
  - When I wanted to search for combinations of words.
  - When the info is scattered across multiple messages, which form a reply chain.
  - When there are many results, it is inconvenient to scroll through them in a tiny search results bar.

## Design choices

- Why WASM?
  - To maintain privacy by keeping all data client-side.
  - To avoid round-trips for queries and data upload.
  - I didn't want to spend money on a backend.
  - Because it is a cool technology and I wanted to try it out.
- Why no embedded db?
  - Because I wanted bespoke lemmatization logic.
  - I also wanted to keep the app lightweight and minimalistic.
- Why Leptos?
  - No reason at all, just wanted to try it out.
  - This project used to use `wasm-bindgen` + vanilla JS + HTML, but I tried doing reactive UI with Leptos and it worked well.
  - Language unification was a nice bonus.
- Why dictionary-based lemmatization?
  - I initially considered using word embeddings, but I could not find a suitable model for Russian.
  - Dictionary gets the work done and does not take too much space (arguably): ≈9MB compressed, ≈300MB uncompressed.


## What I learned

- How to use WASM in a web application.
- How to use Leptos for building a reactive UI.
- Refreshed memories on parsing.

## Potential improvements

- Web Workers for background initialization. Currently it is blocking the main thread.
- Revise the code because it contains a lot of clones and unwraps.

## License

All the code is licensed under MIT License

The file `lemmatization-ru.tsv.gz` in this repository's GitHub releases is a derivative of [OpenCorpora](https://opencorpora.org/?page=downloads)'s Russian language morphologic dictionary and is licenced under [Creative Commons Attribution-ShareAlike 3.0](https://creativecommons.org/licenses/by-sa/3.0/deed.en)

---

Весь код находится под лицензией MIT

Файл `lemmatization-ru.tsv.gz` в GitHub-релизах этого репозитория является производной от морфологического словаря [OpenCorpora](https://opencorpora.org/?page=downloads) и находится под лицензией [Creative Commons Attribution-ShareAlike 3.0](https://creativecommons.org/licenses/by-sa/3.0/deed.ru)
