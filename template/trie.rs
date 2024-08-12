/// 字符串字典树


#[derive(Debug, Default)]
struct Node<Type> {
    children: [Option<Box<Node<Type>>>; 26],
    value: Option<Type>,
}

#[derive(Debug, Default)]
struct Trie<Type>
where 
    Type: Default + Copy
{
    root: Node<Type>,
}

impl<Type> Trie<Type>
where  
    Type: Default + Copy,
{
    pub fn new() -> Self {
        Self {
            root: Node::default(),
        }
    }

    pub fn insert(&mut self, key: String, value: Type)
    {
        let mut node = &mut self.root;
        for &c in key.as_bytes() {
            let next = &mut node.children[(c - b'a') as usize];
            node = next.get_or_insert_with(Box::<Node<Type>>::default);
        }
        node.value = Some(value);
    } 

    pub fn get(&self, key: String) -> Option<Type> 
    where
    {
        let mut node = &self.root;
        for c in key.as_bytes() {
            if let Some(next) = &node.children[(c - b'a') as usize] {
                node = next.as_ref();
            } else {
                return None;
            }
        }
        node.value
    }
}
