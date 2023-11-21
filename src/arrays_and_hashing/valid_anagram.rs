use crate::utils::print_pass;

const NAME: &str = "valid-anagram";

fn valid_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut chars = [0; 26];

    for c in 0..s.len() {
        chars[s.as_bytes()[c] as usize - 97] += 1;
        chars[t.as_bytes()[c] as usize - 97] -= 1;
    }
    chars.iter().all(|c| *c == 0)
}


pub fn main() {
    let s = String::from("invalid");
    let t = String::from("anagram");
    assert!(!valid_anagram(s, t), "anagram should fail if strings are not anagram");
    let s = String::from("nagaram");
    let t = String::from("anagram");
    assert!(valid_anagram(s, t), "anagram should be true if strings are anagram");
    print_pass(NAME)
}
