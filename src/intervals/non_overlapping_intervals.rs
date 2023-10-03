use crate::utils::print_pass;

const NAME: &str = "non-overlapping-intervals";
const LINK: &str = "https://leetcode.com/problems/non-overlapping-intervals/";

pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_by_key(|x| x[1]);
    let mut counter = 0;
    let mut prev_end = intervals[0][1];
    for i in 1..intervals.len() {
        let interval = &intervals[i];
        if prev_end > interval[0] {
            counter += 1;
        } else {
            prev_end = interval[1] 
        }
    }
    counter
}

pub fn main() {
    let intervals = vec![vec![1,2],vec![2,3],vec![3,4],vec![1,3]];
    assert_eq!(erase_overlap_intervals(intervals), 1);
    let intervals = vec![vec![1,2],vec![1,2],vec![1,2]];
    assert_eq!(erase_overlap_intervals(intervals), 2);
    let intervals = vec![vec![1,100],vec![11,22],vec![1,11],vec![2,12]];
    assert_eq!(erase_overlap_intervals(intervals), 2);
    print_pass(NAME, LINK)
}
