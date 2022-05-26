use std::collections::BTreeMap;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct Trie<T, Step> {
    value: Option<T>,
    paths: BTreeMap<Step, Trie<T, Step>>,
}

impl<T: Debug, Step: Ord + Clone + Debug> Trie<T, Step> {
    pub fn root() -> Self {
        Self::default()
    }

    /// needs to clone the key, due to the entry api
    pub fn insert(&mut self, steps: &[Step], val: T) {
        if steps.is_empty() {
            return;
        }

        let node = self.paths.entry(steps[0].clone()).or_insert(Self::root());
        let steps = &steps[1..];

        if steps.is_empty() {
            node.value = Some(val)
        } else {
            node.insert(steps, val)
        }
    }

    pub fn exact_match(&self, steps: &[Step]) -> Option<&T> {
        let mut node = self;
        for s in steps {
            node = node.paths.get(s)?;
        }

        node.value.as_ref()
    }

    pub fn best_match(&self, steps: &[Step]) -> Option<&T> {
        let mut node = self;

        for s in steps.iter() {
            node = match node.paths.get(s) {
                Some(n) => n,
                None => break,
            }
        }

        node.value.as_ref()
    }
}

impl<T, Steps> Default for Trie<T, Steps> {
    fn default() -> Self {
        Trie {
            value: None,
            paths: BTreeMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;
    #[test]
    fn insert() {
        let mut trie: Trie<Vec<char>, char> = Trie::root();

        let hello: Vec<char> = "hello".chars().collect();
        let hell: Vec<char> = "hell".chars().collect();
        let hi: Vec<char> = "hi".chars().collect();
        let bye: Vec<char> = "bye".chars().collect();

        trie.insert(&hello, hello.clone());
        trie.insert(&hell, hell.clone());
        trie.insert(&hi, hi.clone());
        trie.insert(&bye, bye.clone());

        // are exactly there
        assert!(trie.exact_match(&hello).is_some());
        assert!(trie.exact_match(&hell).is_some());
        assert!(trie.exact_match(&hi).is_some());
        assert!(trie.exact_match(&bye).is_some());

        // compare structure
        let keys: Vec<_> = trie.paths.keys().collect();
        assert_eq!(keys, vec![&'b', &'h']);

        let h = trie.paths.get(&'h').unwrap();
        let keys: Vec<_> = h.paths.keys().collect();
        assert_eq!(keys, vec![&'e', &'i']);
    }

    #[test]
    fn best_match() {
        let mut trie: Trie<Vec<char>, char> = Trie::root();

        let hello: Vec<char> = "hello".chars().collect();
        let hell: Vec<char> = "hell".chars().collect();
        let hi: Vec<char> = "hi".chars().collect();
        let bye: Vec<char> = "bye".chars().collect();

        trie.insert(&hello, hello.clone());
        trie.insert(&hell, hell.clone());
        trie.insert(&hi, hi.clone());
        trie.insert(&bye, bye.clone());

        let hello_world: Vec<char> = "hello world".chars().collect();

        let best = trie.best_match(&hello_world).unwrap();

        assert_eq!(best, &hello);
    }
}
