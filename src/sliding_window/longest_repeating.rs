use std::collections::HashMap;

use crate::utils::print_pass;

const NAME: &str = "longest-repeating-character-replacement";


pub fn character_replacement(s: String, k: i32) -> i32 {
    let mut count: HashMap<char, i32> = HashMap::new();

    let mut res = 0;
    let mut l = 0;
    let mut maxf = 0;
    for r in 0..s.len() {
        let r_char = s.as_bytes()[r] as char;
        let l_char = s.as_bytes()[l] as char;
        *count.entry(r_char).or_insert(0) += 1;
        maxf = maxf.max(*count.get(&r_char).unwrap());

        while (r - l + 1) as i32 - maxf > k as i32 {
            *count.entry(l_char).or_insert(0) -= 1;
            l += 1;
        }
        res = res.max(r - l + 1);
    }
    res as i32
}

pub fn main() {
    let s: String = String::from("ABAB");
    let k: i32 = 2;
    assert_eq!(character_replacement(s, k), 4);
    let s: String = String::from("AABABBA");
    let k: i32 = 1;
    assert_eq!(character_replacement(s, k), 4);
    let s: String = String::from("ABAA");
    let k: i32 = 0;
    assert_eq!(character_replacement(s, k), 2);
    let s: String = String::from("ABBB");
    let k: i32 = 2;
    assert_eq!(character_replacement(s, k), 4);
    print_pass(NAME)
}
