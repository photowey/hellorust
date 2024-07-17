#[derive(Default, Debug)]
struct Node {
    mark: bool,
    children: [Option<Box<Node>>; 26],
}

impl Node {
    pub fn new() -> Self {
        Node::default()
    }
}

// ----------------------------------------------------------------

/// 字典树
#[derive(Default, Debug)]
pub struct Dictionary {
    /// 根节点
    root: Box<Node>,
}

impl Dictionary {
    pub fn new() -> Self {
        Dictionary::default()
    }
}

impl Dictionary {
    pub fn insert(&mut self, word: &str) {
        let mut sentinel = &mut self.root;

        for ch in word.as_bytes() {
            let index = (ch - b'a') as usize;
            let child = &mut sentinel.children[index];
            sentinel = child.get_or_insert_with(Box::<Node>::default);
        }

        sentinel.mark = true;
    }

    pub fn contains(&self, word: &str) -> bool {
        self.word_node(word).map_or(false, |node| node.mark)
    }

    pub fn start_with(&self, prefix: &str) -> bool {
        self.word_node(prefix).is_some()
    }

    fn word_node(&self, word: &str) -> Option<&Node> {
        let mut sentinel = &self.root;
        for ch in word.as_bytes() {
            let index = (ch - b'a') as usize;
            match &sentinel.children[index] {
                None => return None,
                Some(node) => sentinel = node,
            }
        }

        Some(&sentinel)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_trie() {
        let mut trie = Dictionary::new();

        trie.insert("rust");
        trie.insert("hello");
        trie.insert("hellorust");

        trie.insert("photo");
        trie.insert("wey");
        trie.insert("photowey");

        assert!(trie.contains("hello"));
        assert!(trie.contains("rust"));
        assert!(trie.start_with("rust"));
        assert!(!trie.start_with("photos"));
        assert!(!trie.start_with("photos"));
    }
}