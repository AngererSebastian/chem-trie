use std::collections::BTreeMap;

#[derive(Debug)]
struct Trie<T, Step> {
    value: Option<T>,
    paths: BTreeMap<Step, Trie<T, Step>>,
}

impl<T, Step: Ord + Clone> Trie<T, Step> {
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

        if steps.len() == 1 {
            node.value = Some(val)
        } else {
            node.insert(steps, val)
        }
    }

    pub fn get(&self, steps: &[Step]) -> Option<&T> {
        let mut node = self;
        for s in steps {
            node = node.paths.get(s)?;
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
