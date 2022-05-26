#![feature(let_chains)]
mod elements;
mod trie;

fn main() {
    let trie = elements::element_trie();
    let word = std::env::args().nth(1).unwrap();
    let word: Vec<_> = word.to_lowercase().chars().collect();
    let mut word = &word[..];

    let mut elements = vec![];

    while let Some(e) = trie.best_match(word) {
        elements.push(e);
        word = &word[e.short.len()..];
    }

    if !word.is_empty() {
        println!("NOT POSSIBLE\n=============\nresults:");
    }

    println!("{:#?}", elements);
}
