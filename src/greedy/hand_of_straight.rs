use std::collections::HashMap;
use crate::utils::print_pass;

const NAME: &str = "hand-of-straights";
const LINK: &str = "https://leetcode.com/problems/hand-of-straights/";

pub fn is_n_straight_hand(mut hand: Vec<i32>, group_size: i32) -> bool {
    hand.sort();
    let mut freq = hand.iter().copied().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });
    for h in hand {
        if *freq.get(&h).unwrap() == 0 {
            continue;
        }
        for i in 0..group_size {
            if freq.contains_key(&(h + i)) && *freq.get(&(h + i)).unwrap() > 0 {
                *freq.get_mut(&(h + i)).unwrap() -= 1;
            } else {
                return false;
            }
        }
    }
    true
}

pub fn main() {
    let hand = vec![1,2,3,6,2,3,4,7,8];
    let target = 3;
    assert_eq!(is_n_straight_hand(hand, target), true);
    let hand = vec![1,2,3,4,5];
    let target = 4;
    assert_eq!(is_n_straight_hand(hand, target), false);
    let hand = vec![3,1,2];
    let target = 3;
    assert_eq!(is_n_straight_hand(hand, target), true);
    print_pass(NAME, LINK)
}
