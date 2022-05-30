#![feature(let_chains)]
#![feature(is_some_with)]
use itertools::Itertools;

mod elements;
mod trie;

fn main() {
    let word = std::env::args().nth(1).unwrap();
    let word: Vec<_> = word.to_lowercase().chars().collect();
    let word = &word[..];

    let trie = elements::element_trie();
    let elements: Vec<_> = get_elements(&trie, word).collect();

    if elements.iter().any(|e| e.is_err()) {
        println!("NOT POSSIBLE\n=============\nresults:");
    }

    // squash the errors
    println!("{:#?}", elements);
}

fn get_elements<'a>(
    trie: &'a trie::Trie<elements::Element, char>,
    mut word: &'a [char],
) -> impl Iterator<Item = Result<&'a elements::Element, String>> + 'a {
    std::iter::from_fn(move || {
        if !word.is_empty() {
            // have Err with the char if no element can be found
            let r = trie.best_match(word).ok_or(word[0]);
            // move one character on error else move the length of the symbol
            // to advance the words
            let step = r.map(|e| e.short.len()).unwrap_or(1);
            word = &word[step..];
            Some(r)
        } else {
            None
        }
    })
    // squash adjacent not recognized characters into string sequences for nicer output
    .peekable() // make it possible to inspect the next element before returning the current
    .batching(|elems| {
        if elems.peek().is_some_with(|r| r.is_err()) {
            let folded_err = elems
                // all adjacent errors
                .peeking_take_while(|r| r.is_err())
                // convert those into one big error
                .fold(String::new(), |mut s, e| {
                    s.push(e.unwrap_err());
                    s
                });
            Some(Err(folded_err))
        } else {
            // non errors -> elements and none
            // just return it, and convert the error into a string because of type matching
            elems.next().map(|r| r.map_err(|e| e.into()))
        }
    })
}
