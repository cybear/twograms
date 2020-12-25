# Twograms

An **n-gram** is a sequence of N words. For instance a bigram (or 2-gram) is the simplest form of an n-gram implementation. 
N-grams can be used for **word predictions**, for instance when you type "I love" and the keyboard suggests "you" for the next word.

Twograms is an n-grams implementation in Rust with the following limitations:

- Currently only 2-grams.
- Performance can be improved in `fn keyval_hashmap_to_wordpredictions`.
- Only the English alphabet is currently supported.
- Not yet exporting to Webassembly
- Not tested on large bodies of text. I imagine that, say, only the top 5 predictions are useful to keep for most use cases
- Currently I'm unsure how to expose the API.

Test data

- A chapter from Alice in Wonderland
- The Bible, King James edition from http://www.gutenberg.org/files/10900/10900-8.txt

