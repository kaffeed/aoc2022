use std::fs;

#[allow(dead_code)]
#[derive(PartialEq, Eq, Debug, Clone)]
enum Type {
    File(String, i32),
    Directory(String),
}

#[allow(dead_code)]
fn read_input(file_name: &str) -> Vec<String> {
    let res: Vec<_> = fs::read_to_string(file_name)
        .expect("file should exist!")
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect();
    res
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct ArenaTree<T>
where
    T: PartialEq,
{
    nodes: Vec<Node<T>>,
}

impl<T> ArenaTree<T>
where
    T: PartialEq,
{
    fn new() -> Self {
        Self { nodes: vec![] }
    }

    fn insert(&mut self, value: T, parent: Option<usize>) {
        let idx = self.nodes.len();
        let node = Node::new(value, idx, parent);
        self.nodes.push(node);
        let Some(parent_idx) = parent else { return };
        let Some(parentnode) = self.nodes.get_mut(parent_idx) else {return};
        parentnode.children.push(idx);
    }

    fn remove(&mut self, idx: usize) -> Result<(), String> {
        let node = self.nodes.remove(idx);
        let Some(parent_idx) = node.parent else { return Ok(()); };
        let Some(parent_node) = self.nodes.get_mut(parent_idx) else { return Ok(()) };
        let idx_child = parent_node
            .children
            .iter()
            .find(|&&x| x == node.idx)
            .unwrap();
        parent_node.children.remove(*idx_child);
        Ok(())
    }

    fn mark_deleted(node_idx: usize) -> Result<(), String> {
        Ok(())
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Node<T>
where
    T: PartialEq,
{
    data: T,
    idx: usize,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(data: T, idx: usize, parent: Option<usize>) -> Self {
        Self {
            data,
            idx,
            parent,
            children: vec![],
        }
    }
}

pub fn run() {
    // let input = read_input("input_day_seven.txt");
    let mut arena: ArenaTree<Type> = ArenaTree::new();

    arena.insert(Type::Directory(String::from("/")), None);
    arena.insert(Type::File(String::from("/test.log"), 24), Some(0));
    arena.insert(Type::File(String::from("/test.log"), 48), Some(0));
    arena.insert(Type::Directory(String::from("test")), Some(0));
    arena.insert(Type::File(String::from("/test.log"), 96), Some(4));

    // arena.insert(Type::Directory, None);

    println!("{arena:#?}");
}
