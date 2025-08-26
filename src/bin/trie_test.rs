use std::{collections::HashMap, time::Instant};

use dmf::string::trie::*;




pub fn main() {
    let start_time = Instant::now();
    let word_list = std::fs::read_to_string("word_list.txt").unwrap();
    let words = word_list.lines();
    let mut trie = Trie::new();
    for word in words {
        trie.insert(word, ());
    }
    let stat = trie.stat();
    println!("Stat: {stat:#?}");
    println!("Size of TrieNode<()>: {}", std::mem::size_of::<TrieNode<()>>());
    let mut max_children = 0usize;
    let max_children_mut = &mut max_children;
    let mut cur_prefix = String::new();
    let curpref = &mut cur_prefix;
    let mut single_node_repeat = 0;
    let snr = &mut single_node_repeat;
    let mut snr_counts = HashMap::<usize, usize>::new();
    let snrc = &mut snr_counts;
    trie.visit_nodes("", move |_, prefix, node| {
        match node {
            TrieNode::Branch(branch)
            | TrieNode::BranchLeaf(branch, _)=> {
                *max_children_mut = (*max_children_mut).max(branch.len());
                if prefix.starts_with(&*curpref)
                && branch.len() == 1 {
                    *snr += 1;
                    curpref.push_str(&prefix[curpref.len()..]);
                } else {
                    (*snrc.entry(*snr).or_insert(0)) += 1;
                    *snr = 0;
                    curpref.clear();
                }
            },
            _ => (),
        }
    });
    let mut counts = Vec::new();
    snr_counts.iter().for_each(|(&length, &count)| {
        counts.push((length, count));
    });
    counts.sort();
    let mut total_savings = 0;
    for (length, count) in counts {
        println!("Len: {length:>3} Count: {count}");
        total_savings += length * count;
    }
    println!("Total Savings with Compression: {total_savings}");
    println!("Memory Savings: {}", total_savings * size_of::<HashMap<char, TrieNode<()>>>());
    println!("Max Children: {}", max_children);
    println!("Printing words with prefix \"dere\"");
    let mut counter = 0;
    trie.visit_leaves("dere", |_, key, _| {
        println!("{key}");
        counter += 1;
    });
    let dere_count = trie.get_node("dere").unwrap().stat().leaf_count;
    println!("{dere_count}");
    assert_eq!(dere_count, counter);
    let mut one_node_count = 0;
    let counter = &mut one_node_count;
    let visit_nodes_timer = Instant::now();
    trie.visit_nodes("", move |_, _, node| {
        match node {
            TrieNode::Branch(branch)
            | TrieNode::BranchLeaf(branch, _) => if branch.len() == 1 {
                *counter += 1;
            },
            _ => ()
        }
    });
    let visit_elapsed = visit_nodes_timer.elapsed();
    println!("Visited Nodes in: {visit_elapsed:.3?}");
    println!("Single Child Count: {one_node_count}");
    let elapsed_time = start_time.elapsed();
    println!("Total Elapsed Time: {elapsed_time:.3?}");
}