#![feature(let_chains)]
mod elements;
mod trie;

fn main() {
    let trie = elements::element_trie();
    let word = std::env::args().nth(1).unwrap();
    let word: Vec<_> = word.to_lowercase().chars().collect();
    let mut word = &word[..];

    let mut elements = vec![];

    while !word.is_empty() {
        // have Err with the char if no element can be found
        let r = trie.best_match(word).ok_or(word[0]);
        elements.push(r);
        // move one character on error else move the length of the symbol
        // to advance the words
        let step = r.map(|e| e.short.len()).unwrap_or(1);
        word = &word[step..];
    }

    // was a success if no error sequence happened
    if elements.iter().any(|e| e.is_err()) {
        println!("NOT POSSIBLE\n=============\nresults:");
    }

    println!("{:#?}", elements);
}
