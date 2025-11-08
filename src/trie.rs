use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_word: false,
        }
    }
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let word = word.trim();

        let mut current_node = &mut self.root;
        for c in word.chars() {
            let next_node = current_node.children.entry(c).or_insert_with(TrieNode::new);
            current_node = next_node;
        }
        current_node.is_word = true;
    }

    /// Find all words with given prefix
    pub fn find_completions(&self, prefix: &str) -> Vec<String> {
        let mut results = Vec::new();

        let mut current = &self.root;
        for c in prefix.chars() {
            match current.children.get(&c) {
                None => return results,
                Some(node) => current = node,
            }
        }

        self.collect_words(current, prefix.to_string(), &mut results);
        results
    }

    /// Recursively collect all words from a node
    fn collect_words(&self, node: &TrieNode, prefix: String, results: &mut Vec<String>) {
        if node.is_word {
            results.push(prefix.clone());
        }

        for (c, child) in &node.children {
            let mut new_prefix = prefix.clone();
            new_prefix.push(*c);
            self.collect_words(child, new_prefix, results);
        }
    }

    #[allow(unused)]
    /// Get single completion if prefix matches exactly one word
    pub fn get_single_completion(&self, prefix: &str) -> Option<String> {
        let completions = self.find_completions(prefix);
        if completions.len() == 1 {
            Some(completions[0].clone())
        } else {
            None
        }
    }
}
