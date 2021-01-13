# Twograms

An **n-gram** is a sequence of N words. For instance a bigram (or 2-gram) is the simplest form of an n-gram implementation. 
N-grams can be used for **word predictions**, for instance when you type "I love" and the keyboard suggests "you" for the next word.

Twograms is an n-grams implementation in Rust with the following limitations:

- Currently only 2-grams.
- Only the English alphabet is currently supported.
- Not yet exporting to Webassembly.
- Currently I'm unsure how to expose the API.


## Test data

For performance testing I'm using the following file saved as `benches/10900-8.txt`: The Bible, King James edition from http://www.gutenberg.org/files/10900/10900-8.txt


## Usage

* `cargo build` builds
* `cargo test` runs the unit tests
* `cargo bench` runs the benchmarks
* `wasm-pack build` builds the WebAssembly binary*

*The WebAssembly binary is actually not faster than normal JS at the moment, so probably not that useful. 
You can measure the performance by firing up a web server in this project's root, for instance `npx http-server`, and visit the page in your browser to try it out. Paste a big body of text into the textarea and pop up dev tools to see how long it takes to process it.

