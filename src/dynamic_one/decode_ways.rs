use crate::utils::print_pass;

const NAME: &str = "decode-ways";
const LINK: &str = "https://leetcode.com/problems/decode-ways/";

pub fn num_decodings(s: String) -> i32 {
    // dp: f(i) => start from i, num of decodings.
    // for every char at i, it can choose to be itself or group with the next
    // f(i) = f(i + 1) ( when s[i] is possible to construct a num ) 
    //      + f(i + 2) ( when s[i], s[i + 1] is possible to construct a num )

    let s_bytes = s.into_bytes();
    let length = s_bytes.len();
    let mut result = vec![0; length];
    for index in (0..length).rev() {
        if s_bytes[index] == b'0' { continue; }
        if index + 1 == length {
            result[index] = 1;
            continue;
        }
        let mut value = result[index + 1];
        if s_bytes[index] == b'1' || (s_bytes[index] == b'2' && s_bytes[index + 1] <= b'6') { 
            if index + 2 < length { 
                value += result[index + 2]; 
            } else { 
                value += 1; 
            }
        }
        result[index] = value;
    }
    result[0]
}

pub fn main() {
    let s = String::from("12");
    assert_eq!(num_decodings(s), 2);
    let s = String::from("226");
    assert_eq!(num_decodings(s), 3);
    let s = String::from("06");
    assert_eq!(num_decodings(s), 0);
    print_pass(NAME, LINK)
}
