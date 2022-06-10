use super::trie::Trie;
use serde::{Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Element {
    pub name: &'static str,
    pub short: &'static str,
    pub neutrons: u16,
    pub protons: u16,
    pub electrons: u16,
}

const ELEMS_STR: &'static str = include_str!("../elements.csv");

pub fn element_trie() -> Trie<Element, char> {
    let elems = get_elements();

    elems.into_iter().fold(Trie::root(), |mut trie, elem| {
        let steps: Vec<_> = elem.short.to_lowercase().chars().collect();
        trie.insert(&steps, elem);
        trie
    })
}

/// reads all the elements from the csv
pub fn get_elements() -> Vec<Element> {
    let lines = ELEMS_STR.lines().skip(1);

    lines.map(element_from_csv).collect()
}

/// creates an element from a csv line like:
/// Element,Symbol,NumberofNeutrons,NumberofProtons,NumberofElectrons
///
/// if the line is not in the format this function panics
fn element_from_csv(line: &'static str) -> Element {
    let mut attrs = line.split(',');

    Element {
        name: attrs.next().unwrap(),
        short: attrs.next().unwrap(),
        neutrons: attrs.next().unwrap().parse().unwrap(),
        protons: attrs.next().unwrap().parse().unwrap(),
        electrons: attrs.next().unwrap().parse().unwrap(),
    }
}
