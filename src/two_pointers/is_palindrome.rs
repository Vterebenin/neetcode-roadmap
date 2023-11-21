use crate::utils::print_pass;

const NAME: &str = "is-palindrome";


fn is_palindrome(s: String) -> bool {
    let iter = s.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase());
    iter.clone().eq(iter.rev())
}


pub fn main() {
    let s = String::from("abc Cbe, ");
    assert!(!is_palindrome(s));
    let s = String::from("abc Cba");
    assert!(is_palindrome(s));
    let s = String::from("0P");
    assert!(!is_palindrome(s));
    print_pass(NAME)
}
