use std::collections::HashMap;

use crate::utils::print_pass;

const NAME: &str = "time-based-key-value-store";


#[derive(Debug, Clone)]
struct TimeMap {
    items: HashMap<String, Vec<(i32, String)>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

pub fn find_nearest_left(arr: &Vec<(i32, String)>, target: i32) -> Option<&(i32, String)> {
    match arr.binary_search_by(|(key, _)| key.cmp(&target)) {
        Ok(index) => Some(&arr[index]), // Exact match
        Err(index) => {
            if index > 0 {
                Some(&arr[index - 1]) // Nearest element on the left
            } else {
                None // No elements on the left
            }
        }
    }
}

impl TimeMap {

    fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
    
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        let entry = self.items.entry(key).or_insert(Vec::new());
        entry.push((timestamp, value));
    }
    
    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(items) = self.items.get(&key) {
            let closest = find_nearest_left(&items, timestamp);
            if closest.is_some() {
                return closest.unwrap().1.clone();
            }
        };
        "".to_string()
    }
}

pub fn main() {
    let mut obj = TimeMap::new();
    obj.set("foo".to_string(), "bar".to_string(), 1);
    obj.set("foo".to_string(), "bar".to_string(), 2);
    obj.set("foo".to_string(), "bar".to_string(), 3);
    obj.set("foo".to_string(), "bxz".to_string(), 7);
    obj.set("foo".to_string(), "bar".to_string(), 12);
    obj.get("foo".to_string(), 10);
    print_pass(NAME)
}
