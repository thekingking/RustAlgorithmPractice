
#[derive(Default)]
struct Trie {
    root: Node,
}

#[derive(Default)]
struct Node {
    children: [Option<Box<Node>>; 26],
    is_word: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    fn new() -> Self {
        Self::default()
        // Trie {
        //     root: Node {
        //         children: [None; 26],
        //         is_word: false,
        //     }
        // }
    }
    
    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.as_bytes() {
            let idx = (c - b'a') as usize;
            let next = &mut node.children[idx];
            node = next.get_or_insert_with(Box::<Node>::default);
        }
        node.is_word = true;
    }
    
    fn search(&self, word: String) -> bool {
        self.get_node(&word).map_or(false, |w| w.is_word)
    }

    fn get_node(&self, s: &str) -> Option<&Node> {
        let mut node = &self.root;
        for c in s.as_bytes() {
            let idx = (c - b'a') as usize;
            if let Some(next) = &node.children[idx] {
                node = next.as_ref();
            } else {
                return None;
            }
        }
        Some(node)
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        self.get_node(&prefix).is_some()
    }
}