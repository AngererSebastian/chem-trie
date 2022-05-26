#![feature(let_chains)]
mod elements;
mod trie;

fn main() {
    let trie = elements::element_trie();
    println!("{:#?}", trie);
}
