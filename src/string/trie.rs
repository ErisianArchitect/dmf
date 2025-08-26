use std::{borrow::Borrow, collections::HashMap, ptr::NonNull};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TrieStat {
    /// The number of empty nodes. This should be zero. If this value
    /// is not zero, that means that the Trie is corrupted. It may not
    /// be broken, but the structure may break recursive node cleanup.
    pub empty_count: usize,
    /// The total node count.
    pub node_count: usize,
    /// The total leaf count.
    pub leaf_count: usize,
    /// The total branch count.
    pub branch_count: usize,
    /// The maximum depth of the tree.
    pub depth: usize,
    /// The total estimated memory usage.
    pub estimated_memory: usize,
}

impl TrieStat {
    pub const fn new() -> Self {
        Self {
            empty_count: 0,
            node_count: 0,
            leaf_count: 0,
            branch_count: 0,
            depth: 0,
            estimated_memory: 0,
        }
    }
    
    #[inline(always)]
    fn add_empty(&mut self) {
        self.empty_count += 1;
    }
    
    #[inline(always)]
    fn add_node(&mut self) {
        self.node_count += 1;
    }
    
    #[inline(always)]
    fn add_leaf(&mut self) {
        self.leaf_count += 1;
    }
    
    #[inline(always)]
    fn add_branch(&mut self) {
        self.branch_count += 1;
    }
    
    #[inline(always)]
    fn add_branchleaf(&mut self) {
        self.add_leaf();
        self.add_branch();
    }
    
    #[inline(always)]
    fn compare_and_set_depth(&mut self, depth: usize) {
        self.depth = self.depth.max(depth);
    }
    
    #[inline(always)]
    fn increase_memory(&mut self, memory: usize) {
        self.estimated_memory += memory;
    }
}

#[derive(Debug, Default, Clone)]
pub enum TrieNode<T> {
    #[default]
    Empty,
    Leaf(T),
    Branch(HashMap<char, TrieNode<T>>),
    BranchLeaf(HashMap<char, TrieNode<T>>, T),
}

impl<T> TrieNode<T> {
    
    /// Returns (leaf, is_empty_node)
    pub fn remove_leaf(&mut self) -> Option<(T, bool)> {
        unsafe {
            let current = std::ptr::read(self);
            let mut result = None;
            std::ptr::write(self, match current {
                TrieNode::Leaf(old) => {
                    result = Some((old, true));
                    TrieNode::Empty
                }
                TrieNode::BranchLeaf(branch, old) => {
                    result = Some((old, false));
                    TrieNode::Branch(branch)
                }
                node => node,
            });
            result
        }
    }
    
    pub fn remove_node(&mut self, chr: char) -> Option<(Self, bool)> {
        unsafe {
            let current = std::ptr::read(self);
            let mut result = None;
            std::ptr::write(self, match current {
                TrieNode::Branch(mut branch) => {
                    let node = branch.remove(&chr);
                    if branch.is_empty() {
                        result = node.map(|node| (node, true));
                        TrieNode::Empty
                    } else {
                        result = node.map(|node| (node, false));
                        TrieNode::Branch(branch)
                    }
                    
                }
                TrieNode::BranchLeaf(mut branch, leaf) => {
                    let node = branch.remove(&chr);
                    result = node.map(|node| (node, false));
                    if branch.is_empty() {
                        TrieNode::Leaf(leaf)
                    } else {
                        TrieNode::BranchLeaf(branch, leaf)
                    }
                }
                node => node,
            });
            result
        }
    }
    
    pub fn set_leaf(&mut self, value: T) -> Option<T> {
        unsafe {
            let current = std::ptr::read(self);
            let mut result = None;
            std::ptr::write(self, match current {
                TrieNode::Empty => TrieNode::Leaf(value),
                TrieNode::Leaf(old) => {
                    result = Some(old);
                    TrieNode::Leaf(value)
                }
                TrieNode::Branch(hash_map) => TrieNode::BranchLeaf(hash_map, value),
                TrieNode::BranchLeaf(hash_map, old) => {
                    result = Some(old);
                    TrieNode::BranchLeaf(hash_map, value)
                }
            });
            result
        }
    }
    
    pub fn get_or_insert_with<F: FnOnce() -> Self>(&mut self, chr: char, or_insert: F) -> &mut Self {
        unsafe {
            let current = std::ptr::read(self);
            std::ptr::write(self, match current {
                TrieNode::Empty => {
                    let init_map = HashMap::new();
                    Self::Branch(init_map)
                }
                TrieNode::Leaf(leaf) => {
                    let init_map = HashMap::new();
                    Self::BranchLeaf(init_map, leaf)
                }
                TrieNode::Branch(hash_map) => Self::Branch(hash_map),
                TrieNode::BranchLeaf(hash_map, leaf) => Self::BranchLeaf(hash_map, leaf),
            });
            match self {
                Self::Branch(map) => {
                    map.entry(chr).or_insert_with(or_insert)
                }
                Self::BranchLeaf(map, _) => {
                    map.entry(chr).or_insert_with(or_insert)
                }
                _ => unreachable!(),
            }
        }
    }
    
    pub fn has_leaf(&self) -> bool {
        match self {
            TrieNode::Empty => false,
            TrieNode::Leaf(_) => true,
            TrieNode::Branch(_) => false,
            TrieNode::BranchLeaf(_, _) => true,
        }
    }
    
    pub fn has_branch(&self) -> bool {
        match self {
            TrieNode::Empty => false,
            TrieNode::Leaf(_) => false,
            TrieNode::Branch(_) => true,
            TrieNode::BranchLeaf(_, _) => true,
        }
    }
    
    /// Returns true if the node is [TrieNode::Empty] or if the branch is empty and there is no leaf.
    pub fn is_empty(&self) -> bool {
        match self {
            TrieNode::Empty => true,
            TrieNode::Leaf(_) => false,
            TrieNode::Branch(hash_map) => hash_map.is_empty(),
            TrieNode::BranchLeaf(_, _) => false,
        }
    }
    
    pub fn get_leaf_value(&self) -> Option<&T> {
        match self {
            TrieNode::Empty => None,
            TrieNode::Leaf(leaf) => Some(leaf),
            TrieNode::Branch(_) => None,
            TrieNode::BranchLeaf(_, leaf) => Some(leaf),
        }
    }
    
    pub fn get_leaf_value_mut(&mut self) -> Option<&mut T> {
        match self {
            TrieNode::Empty => None,
            TrieNode::Leaf(leaf) => Some(leaf),
            TrieNode::Branch(_) => None,
            TrieNode::BranchLeaf(_, leaf) => Some(leaf),
        }
    }
    
    pub fn get_leaf<S: Borrow<str>>(&self, partial_key: S) -> Option<&T> {
        let chars = partial_key.borrow().chars();
        let mut node = self;
        for chr in chars {
            node = node.get(chr)?;
        }
        node.get_leaf_value()
    }
    
    pub fn get_leaf_mut<S: Borrow<str>>(&mut self, partial_key: S) -> Option<&mut T> {
        let chars = partial_key.borrow().chars();
        let mut node = self;
        for chr in chars {
            node = node.get_mut(chr)?;
        }
        node.get_leaf_value_mut()
    }
    
    pub fn get_node<S: Borrow<str>>(&self, partial_key: S) -> Option<&Self> {
        let chars = partial_key.borrow().chars();
        let mut node = self;
        for chr in chars {
            node = node.get(chr)?;
        }
        Some(node)
    }
    
    pub fn get_node_mut<S: Borrow<str>>(&mut self, partial_key: S) -> Option<&mut Self> {
        let chars = partial_key.borrow().chars();
        let mut node = self;
        for chr in chars {
            node = node.get_mut(chr)?;
        }
        Some(node)
    }
    
    pub fn get(&self, chr: char) -> Option<&Self> {
        match self {
            TrieNode::Empty => None,
            TrieNode::Leaf(_) => None,
            TrieNode::Branch(hash_map) => hash_map.get(&chr),
            TrieNode::BranchLeaf(hash_map, _) => hash_map.get(&chr),
        }
    }
    
    pub fn get_mut(&mut self, chr: char) -> Option<&mut Self> {
        match self {
            TrieNode::Empty => None,
            TrieNode::Leaf(_) => None,
            TrieNode::Branch(hash_map) => hash_map.get_mut(&chr),
            TrieNode::BranchLeaf(hash_map, _) => hash_map.get_mut(&chr),
        }
    }
    
    fn visit_internal<F: FnMut(usize, &str, &Self)>(&self, prefix_builder: &mut String, depth: usize, visitor: &mut F) {
        visitor(depth, prefix_builder, self);
        match self {
            TrieNode::Branch(branch)
            | TrieNode::BranchLeaf(branch, _) => {
                branch.iter().for_each(move |(&chr, node)| {
                    prefix_builder.push(chr);
                    node.visit_internal(prefix_builder, depth + 1, visitor);
                    prefix_builder.pop();
                })
            }
            _ => (),
        }
    }
    
    fn visit_internal_mut<F: for<'a> FnMut(usize, &str, &'a mut Self)>(&mut self, prefix_builder: &mut String, depth: usize, visitor: &mut F) {
        visitor(depth, prefix_builder, self);
        match self {
            TrieNode::Branch(branch)
            | TrieNode::BranchLeaf(branch, _) => {
                branch.iter_mut().for_each(move |(&chr, node)| {
                    prefix_builder.push(chr);
                    node.visit_internal_mut(prefix_builder, depth + 1, visitor);
                    prefix_builder.pop();
                })
            }
            _ => ()
        }
    }
    
    pub fn visit_nodes<F: FnMut(usize, &str, &Self)>(&self, depth: usize, mut visitor: F) {
        let mut prefix_builder = String::new();
        self.visit_internal(&mut prefix_builder, depth, &mut visitor);
    }
    
    pub fn visit_nodes_mut<F: FnMut(usize, &str, &mut Self)>(&mut self, depth: usize, mut visitor: F) {
        let mut prefix_builder = String::new();
        self.visit_internal_mut(&mut prefix_builder, depth, &mut visitor);
    }
    
    pub fn visit_leaves<F: FnMut(usize, &str, &T)>(&self, mut visitor: F) {
        let mut prefix_builder = String::new();
        self.visit_internal(&mut prefix_builder, 0, &mut move |depth, prefix, node| {
            match node {
                TrieNode::Leaf(leaf)
                | TrieNode::BranchLeaf(_, leaf) => visitor(depth, prefix, leaf),
                _ => (),
            }
        });
    }
    
    pub fn visit_leaves_mut<F: FnMut(usize, &str, &mut T)>(&mut self, mut visitor: F) {
        let mut prefix_builder = String::new();
        self.visit_internal_mut(&mut prefix_builder, 0, &mut move |depth, prefix, node| {
            match node {
                TrieNode::Leaf(leaf)
                | TrieNode::BranchLeaf(_, leaf) => visitor(depth, prefix, leaf),
                _ => (),
            }
        });
    }
    
    fn find_node_internal<'a, R: 'a, F: FnMut(usize, &str, &'a Self) -> Option<R>>(&'a self, prefix_builder: &mut String, depth: usize, find: &mut F) -> Option<R> {
        find(depth, prefix_builder, self).or_else(move || {
            match self {
                TrieNode::Branch(branch)
                | TrieNode::BranchLeaf(branch, _) => {
                    branch.iter().find_map(move |(&chr, node)| {
                        prefix_builder.push(chr);
                        let result = node.find_node_internal(prefix_builder, depth + 1, find);
                        prefix_builder.pop();
                        result
                    })
                }
                _ => None
            }
        })
    }
    
    pub fn find_node<'a, R: 'a, F: FnMut(usize, &str, &'a Self) -> Option<R>>(&'a self, depth: usize, mut search: F) -> Option<R> {
        let mut prefix_builder = String::new();
        self.find_node_internal(&mut prefix_builder, depth, &mut search)
    }
    
    fn find_leaf_internal<'a, R: 'a, F: FnMut(usize, &str, &'a T) -> Option<R>>(&'a self, prefix_builder: &mut String, depth: usize, find: &mut F) -> Option<R> {
        match self {
            TrieNode::Empty => None,
            TrieNode::Leaf(leaf) => find(depth, prefix_builder, leaf),
            TrieNode::Branch(branch) => {
                branch.iter().find_map(move |(&chr, node)| {
                    prefix_builder.push(chr);
                    let result = node.find_leaf_internal(prefix_builder, depth + 1, find);
                    prefix_builder.pop();
                    result
                })
            }
            TrieNode::BranchLeaf(branch, leaf) => {
                find(depth, prefix_builder, leaf).or_else(move || {
                    branch.iter().find_map(move |(&chr, node)| {
                        prefix_builder.push(chr);
                        let result = node.find_leaf_internal(prefix_builder, depth + 1, find);
                        prefix_builder.pop();
                        result
                    })
                })
            }
        }
    }
    
    fn find_leaf_internal_mut<'a, R: 'a, F: FnMut(usize, &str, &'a mut T) -> Option<R>>(&'a mut self, prefix_builder: &mut String, depth: usize, find: &mut F) -> Option<R> {
        match self {
            TrieNode::Empty => None,
            TrieNode::Leaf(leaf) => find(depth, prefix_builder, leaf),
            TrieNode::Branch(branch) => {
                branch.iter_mut().find_map(move |(&chr, node)| {
                    prefix_builder.push(chr);
                    let result = node.find_leaf_internal_mut(prefix_builder, depth + 1, find);
                    prefix_builder.pop();
                    result
                })
            }
            TrieNode::BranchLeaf(branch, leaf) => {
                find(depth, prefix_builder, leaf).or_else(move || {
                    branch.iter_mut().find_map(move |(&chr, node)| {
                        prefix_builder.push(chr);
                        let result = node.find_leaf_internal_mut(prefix_builder, depth + 1, find);
                        prefix_builder.pop();
                        result
                    })
                })
            }
        }
    }
    
    pub fn find_leaf<'a, R: 'a, F: FnMut(usize, &str, &'a T) -> Option<R>>(&'a self, depth: usize, mut find: F) -> Option<R> {
        let mut prefix_builder = String::new();
        self.find_leaf_internal(&mut prefix_builder, depth, &mut find)
    }
    
    pub fn find_leaf_mut<'a, R: 'a, F: FnMut(usize, &str, &'a mut T) -> Option<R>>(&'a mut self, depth: usize, mut find: F) -> Option<R> {
        let mut prefix_builder = String::new();
        self.find_leaf_internal_mut(&mut prefix_builder, depth, &mut find)
    }
    
    pub fn stat(&self) -> TrieStat {
        let mut stat = TrieStat::new();
        let stat_mut = &mut stat;
        stat_mut.increase_memory(size_of_val(self));
        self.visit_nodes(0, move |depth, _, node| {
            stat_mut.add_node();
            stat_mut.compare_and_set_depth(depth);
            match node {
                TrieNode::Empty => stat_mut.add_empty(),
                TrieNode::Leaf(_) => stat_mut.add_leaf(),
                TrieNode::Branch(branch) => {
                    stat_mut.add_branch();
                    stat_mut.increase_memory(branch.capacity() * size_of::<(char, TrieNode<T>)>());
                }
                TrieNode::BranchLeaf(branch, _) => {
                    stat_mut.add_branchleaf();
                    stat_mut.increase_memory(branch.capacity() * size_of::<(char, TrieNode<T>)>());
                }
            }
        });
        stat
    }
}

// For Trie removal backtracking.
struct Back<T> {
    node: NonNull<TrieNode<T>>,
    remove_char: char,
}
impl<T> Back<T> {
    #[must_use]
    #[inline(always)]
    fn new(node: NonNull<TrieNode<T>>, remove_char: char) -> Self {
        Self {
            node: NonNull::from(node),
            remove_char,
        }
    }
    
    unsafe fn as_mut(&mut self) -> &mut TrieNode<T> {
        unsafe {
            self.node.as_mut()
        }
    }
}

struct Backtracker<T> {
    stack: Vec<Back<T>>,
}

impl<T> Backtracker<T> {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }
    
    fn push(&mut self, node: NonNull<TrieNode<T>>, rem_char: char) {
        self.stack.push(Back::new(node, rem_char))
    }
    
    /// Pops the next node from the stack for removal. Returns true when there are no more nodes to remove.
    fn backtrack(&mut self) {
        loop {
            let Some(mut top) = self.stack.pop() else {
                return;
            };
            unsafe {
                let rem_char = top.remove_char;
                if let Some((_, remove)) = top.as_mut().remove_node(rem_char) {
                    if !remove || self.stack.is_empty() {
                        return;
                    }
                }
            }
        }
    }
}

/// A prefix tree for string matching.
#[derive(Debug, Default, Clone)]
pub struct Trie<T> {
    root: TrieNode<T>,
}

impl<T> Trie<T> {
    pub fn new() -> Self {
        Self {
            root: TrieNode::Empty,
        }
    }
    
    pub fn insert(&mut self, key: &str, value: T) -> Option<T> {
        let chars = key.chars();
        let mut node = &mut self.root;
        for chr in chars {
            node = node.get_or_insert_with(chr, || TrieNode::Empty);
        }
        node.set_leaf(value)
    }
    
    pub fn get_node<S: Borrow<str>>(&self, partial_key: S) -> Option<&TrieNode<T>> {
        let chars = partial_key.borrow().chars();
        let mut node = &self.root;
        for chr in chars {
            node = node.get(chr)?;
        }
        Some(node)
    }
    
    pub fn get_node_mut<S: Borrow<str>>(&mut self, partial_key: S) -> Option<&mut TrieNode<T>> {
        let chars = partial_key.borrow().chars();
        let mut node = &mut self.root;
        for chr in chars {
            node = node.get_mut(chr)?;
        }
        Some(node)
    }
    
    pub fn get_leaf<S: Borrow<str>>(&self, key: S) -> Option<&T> {
        let chars = key.borrow().chars();
        let mut node = &self.root;
        for chr in chars {
            node = node.get(chr)?;
        }
        node.get_leaf_value()
    }
    
    pub fn get_leaf_mut<S: Borrow<str>>(&mut self, key: S) -> Option<&mut T> {
        let chars = key.borrow().chars();
        let mut node = &mut self.root;
        for chr in chars {
            node = node.get_mut(chr)?;
        }
        node.get_leaf_value_mut()
    }
    
    pub fn contains<S: Borrow<str>>(&self, key: S) -> bool {
        self.get_leaf(key).is_some()
    }
    
    pub fn contains_partial<S: Borrow<str>>(&self, partial_key: S) -> bool {
        self.get_node(partial_key).is_some()
    }
    
    pub fn remove_leaf<S: Borrow<str>>(&mut self, key: S) -> Option<T> {
        let mut backtracker = Backtracker::<T>::new();
        unsafe {
            let mut node = NonNull::<TrieNode<T>>::from(&self.root);
            let chars = key.borrow().chars();
            for chr in chars {
                let child = node.as_mut().get(chr)?;
                backtracker.push(node, chr);
                node = NonNull::from(child);
            }
            // Now we should be at the top.
            if let Some((leaf, empty)) = node.as_mut().remove_leaf() {
                if empty {
                    backtracker.backtrack();
                }
                Some(leaf)
            } else {
                None
            }
        }
    }
    
    pub fn clear(&mut self) {
        // It's that easy.
        self.root = TrieNode::Empty
    }
    
    pub fn is_empty(&self) -> bool {
        matches!(self.root, TrieNode::Empty)
    }
    
    pub fn visit_nodes<F: FnMut(usize, &str, &TrieNode<T>)>(&self, prefix: &str, mut visitor: F) {
        if let Some(node) = self.get_node(prefix) {
            let mut prefix_builder = String::from(prefix);
            node.visit_internal(&mut prefix_builder, 0, &mut visitor);
        }
    }
    
    pub fn visit_nodes_mut<F: FnMut(usize, &str, &mut TrieNode<T>)>(&mut self, prefix: &str, mut visitor: F) {
        if let Some(node) = self.get_node_mut(prefix) {
            let mut prefix_builder = String::from(prefix);
            node.visit_internal_mut(&mut prefix_builder, 0, &mut visitor);
        }
    }
    
    pub fn visit_leaves<F: FnMut(usize, &str, &T)>(&self, prefix: &str, mut visitor: F) {
        if let Some(node) = self.get_node(prefix) {
            let mut prefix_builder = String::from(prefix);
            node.visit_internal(&mut prefix_builder, 0, &mut move |depth, prefix, node| {
                match node {
                    TrieNode::Leaf(leaf)
                    | TrieNode::BranchLeaf(_, leaf) => visitor(depth, prefix, leaf),
                    _ => (),
                }
            });
        }
    }
    
    pub fn visit_leaves_mut<F: FnMut(usize, &str, &mut T)>(&mut self, prefix: &str, mut visitor: F) {
        if let Some(node) = self.get_node_mut(prefix) {
            let mut prefix_builder = String::from(prefix);
            node.visit_internal_mut(&mut prefix_builder, 0, &mut move |depth, prefix, node| {
                match node {
                    TrieNode::Leaf(leaf)
                    | TrieNode::BranchLeaf(_, leaf) => visitor(depth, prefix, leaf),
                    _ => (),
                }
            });
        }
    }
    
    pub fn find_leaf<'a, R: 'a, F: FnMut(usize, &str, &'a T) -> Option<R>>(&'a self, prefix: &str, mut find: F) -> Option<R> {
        if let Some(node) = self.get_node(prefix) {
            let mut prefix_builder = String::from(prefix);
            node.find_leaf_internal(&mut prefix_builder, 0, &mut find)
        } else {
            None
        }
    }
    
    pub fn find_leaf_mut<'a, R: 'a, F: FnMut(usize, &str, &'a mut T) -> Option<R>>(&'a mut self, prefix: &str, mut find: F) -> Option<R> {
        if let Some(node) = self.get_node_mut(prefix) {
            let mut prefix_builder = String::from(prefix);
            node.find_leaf_internal_mut(&mut prefix_builder, 0, &mut find)
        } else {
            None
        }
    }
    
    pub fn stat(&self) -> TrieStat {
        let mut stat = TrieStat::new();
        let stat_mut = &mut stat;
        stat_mut.increase_memory(size_of_val(self));
        self.visit_nodes("", move |depth, _, node| {
            stat_mut.add_node();
            stat_mut.compare_and_set_depth(depth);
            match node {
                TrieNode::Empty => stat_mut.add_empty(),
                TrieNode::Leaf(_) => stat_mut.add_leaf(),
                TrieNode::Branch(branch) => {
                    stat_mut.add_branch();
                    stat_mut.increase_memory(branch.capacity() * size_of::<(char, TrieNode<T>)>());
                }
                TrieNode::BranchLeaf(branch, _) => {
                    stat_mut.add_branchleaf();
                    stat_mut.increase_memory(branch.capacity() * size_of::<(char, TrieNode<T>)>());
                }
            }
        });
        stat
    }
    
    pub fn count_leaves(&self) -> usize {
        self.stat().leaf_count
    }
    
    pub fn count_nodes(&self) -> usize {
        self.stat().node_count
    }
    
    pub fn count_branches(&self) -> usize {
        self.stat().branch_count
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn trie_test() {
        const WORDS: &[&str] = &[
            "hello",
            "help",
            "health",
            "world",
            "World",
            "work",
            "working",
            "worker",
            "word",
            "wordle",
            "wardrobe",
            "wartorn",
            "warhorse",
            "wartown",
        ];
        let mut trie = Trie::new();
        for &word in WORDS {
            trie.insert(word, word);
        }
        
        trie.visit_leaves("wor", |_, key, _| {
            println!("wor: {key}");
        });
        
        for &word in WORDS {
            if let Some(&word) = trie.get_leaf(word) {
                println!("Word: {word}");
            }
        }
        let partial = trie.get_node("war").unwrap();
        println!("{}", "*".repeat(64));
        if let Some(&word) = partial.get_leaf("town") {
            println!("{word}");
        }
        
        let result = trie.find_leaf("", |_, key, &leaf| {
            if leaf.contains("town") {
                Some(key.to_owned())
            } else {
                None
            }
        });
        assert!(result.is_some());
        println!("{result:?}");
        
        let stat = trie.stat();
        println!("Stat: {stat:#?}");
        
        let result = trie.find_leaf_mut("", move |_, key, node| {
            if node.contains("work") {
                Some(key.to_owned())
            } else {
                None
            }
        });
        println!("{:?}", result);
        
        println!("{trie:#?}");
        
        assert!(!trie.is_empty());
        for &word in WORDS {
            trie.remove_leaf(word);
        }
        assert!(trie.is_empty());
    
    }
}