//! # 字典树 (前缀树)
//!
//!

use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Default)]
struct Node<K, V> {
    children: HashMap<K, Node<K, V>>,
    value: Option<V>,
}

pub struct Trie<K, V> {
    root: Node<K, V>,
}

impl<K, V> Trie<K, V>
where
    K: Eq + Hash + Default,
    V: Default,
{
    pub fn new() -> Self {
        Self {
            root: Node::default(),
        }
    }

    pub fn insert(&mut self, word: impl IntoIterator<Item = K>, value: V) {
        let mut node = &mut self.root;
        for c in word {
            node = node.children.entry(c).or_default();
        }
        node.value = Some(value);
    }

    pub fn query(&self, word: impl IntoIterator<Item = K>) -> Option<&V> {
        let mut node = &self.root;
        for c in word {
            if node.children.contains_key(&c) {
                node = node.children.get(&c).unwrap();
            } else {
                return None;
            }
        }
        node.value.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query() {
        let mut t = Trie::new();
        t.insert("cat".chars(), 1);
        t.insert("car".chars(), 2);
        t.insert("cate".chars(), 3);
        assert_eq!(t.query("car".chars()), Some(&2));
        assert_eq!(t.query("cost".chars()), None);
    }
}
