use std::collections::HashMap;
use crate::utils::print_pass;

const NAME: &str = "implement-trie-prefix-tree";
const LINK: &str = "https://leetcode.com/problems/implement-trie-prefix-tree/";

#[derive(Debug, PartialEq, Eq)]
struct Trie {
    is_word_end: bool,
    children: Option<HashMap<char, Trie>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    fn new() -> Self {
        Trie {
            is_word_end: false,
            children: Some(HashMap::new()),
        }
    }
    fn find_by_str(&self, str: String) {
    }
    fn insert_after() {
        
    }

    fn insert(&mut self, word: String) {
        let mut word_copy = word.clone();
        let mut trie = self;
        while !word_copy.is_empty() {
            let letter_key = word_copy.remove(0).to_lowercase().collect::<Vec<char>>();
            if let Some(ref children) = &trie.children {
                let item = children.get(&letter_key[0]);
                println!("{:?}", &item);
            }
                // .or_insert(Trie { 
                //     is_word_end: word_copy.is_empty(),
                //     children: Some(HashMap::new()),
                // }).children.unwrap();
        }
    }
    
    fn search(&self, word: String) -> bool {
        true
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
pub fn main() {
    let mut trie = Trie::new();
    trie.insert("Woah".to_string());
    print_pass(NAME, LINK);
}


