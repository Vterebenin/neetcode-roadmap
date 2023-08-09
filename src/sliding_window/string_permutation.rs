use crate::utils::print_pass;

const NAME: &str = "permutation-in-string";
const LINK: &str = "https://leetcode.com/problems/permutation-in-string/";

pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }

    let mut s1_count: [usize; 26] = [0; 26];
    let mut s2_count: [usize; 26] = [0; 26];

    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();

    for i in 0..s1.len() {
        s1_count[(s1_bytes[i] - b'a') as usize] += 1;
        s2_count[(s2_bytes[i] - b'a') as usize] += 1;
    }

    let mut matches: u8 = 0;
    for i in 0..26 {
        if s1_count[i] == s2_count[i] {
            matches += 1;
        }
    }

    let mut l: usize = 0;
    for r in s1.len()..s2.len() {
        if matches == 26 {
            return true;
        }

        let idx_left = (s2_bytes[l] - b'a') as usize;
        s2_count[idx_left] -= 1;
        if s1_count[idx_left] == s2_count[idx_left] {
            matches += 1;
        } else if s1_count[idx_left] == s2_count[idx_left] + 1 {
            matches -= 1;
        }

        let idx_right = (s2_bytes[r] - b'a') as usize;
        s2_count[idx_right] += 1;
        if s1_count[idx_right] == s2_count[idx_right] {
            matches += 1;
        } else if s1_count[idx_right] == s2_count[idx_right] - 1 {
            matches -= 1;
        }

        l += 1;
    }

    return matches == 26;
}

pub fn main() {
    // let s1: String = String::from("ab");
    // let s2: String = String::from("eidbaooo");
    // assert_eq!(check_inclusion(s1, s2), true);
    let s1: String = String::from("adc");
    let s2: String = String::from("dcda");
    assert_eq!(check_inclusion(s1, s2), true);
    print_pass(NAME, LINK)
}
