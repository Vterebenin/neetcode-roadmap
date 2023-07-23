use colored::Colorize;

fn valid_anagram(s: String, t: String) -> bool {
    true
}

pub fn main() {
    let s = String::from("invalid");
    let t = String::from("anagram");
    assert!(!valid_anagram(s, t), "anagram should fail if strings are not anagram");
    let s = String::from("nagaram");
    let t = String::from("anagram");
    assert!(valid_anagram(s, t));
    println!("{}", "contains_duplicate passed!".green());
}
