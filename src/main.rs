#![feature(let_chains)]
#![feature(is_some_with)]
use itertools::Itertools;
use owo_colors::OwoColorize;
use serde::Serialize;
use structopt::StructOpt;

mod elements;
mod trie;

type LookupResult<'a> = Result<&'a elements::Element, String>;

#[derive(StructOpt)]
struct Opts {
    #[structopt(short, long)]
    json: bool,
    word: String,
}

fn main() {
    let opts = Opts::from_args();
    let word: Vec<_> = opts.word.to_lowercase().chars().collect();
    let word = &word[..];

    let trie = elements::element_trie();
    let elements: Vec<_> = get_elements(&trie, word).collect();

    if opts.json {
        print_json(&elements)
    } else {
        print_pretty(&elements)
    }
}

fn get_elements<'a>(
    trie: &'a trie::Trie<elements::Element, char>,
    mut word: &'a [char],
) -> impl Iterator<Item = LookupResult<'a>> + 'a {
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

fn print_pretty(elements: &[LookupResult<'_>]) {
    for e in elements.iter() {
        match e {
            Ok(e) => print!("{}", e.short.blue()),
            Err(e) => print!("{}", e.red()),
        }
    }

    println!("\n\nDISTINCT elements:\n-----------------");

    let distincs: Vec<_> = elements
        .iter()
        .filter_map(|r| r.as_ref().ok())
        .sorted()
        .dedup()
        .collect();

    let name_len = distincs
        .iter()
        .map(|e| e.name.len())
        .max()
        .unwrap_or_default();

    for &d in distincs {
        println!(
            " {:2} - {:max_name$} protons: {:02}, neutrons: {:02}, electrons: {:02}",
            d.short.blue(),
            d.name.to_string() + ",",
            d.protons.red(),
            d.neutrons.green(),
            d.electrons.yellow(),
            max_name = name_len + 1
        );
    }
}

#[derive(Serialize)]
struct JsonOutput<'a> {
    sequence: std::borrow::Cow<'a, str>,
    matched: Option<&'a elements::Element>,
}

fn print_json(elements: &[LookupResult<'_>]) {
    // do this without serde
    let json = elements
        .iter()
        .map(|elem| match elem {
            Ok(elem) => JsonOutput {
                sequence: elem.short.into(),
                matched: Some(&elem),
            },
            Err(err) => JsonOutput {
                sequence: err.into(),
                matched: None,
            },
        })
        .collect_vec();

    let output = serde_json::to_string(&json).unwrap();

    println!("{}", output);
}
