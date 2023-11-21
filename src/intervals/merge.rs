use crate::utils::print_pass;

const NAME: &str = "merge-intervals";


pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort_by(|x, y| x[0].cmp(&y[0]));
    let mut res = vec![];
    for interval in intervals {
        let prev_interval = match res.last_mut() {
            Some(a) => a,
            None => {
                res.push(interval);
                continue;
            }
        };
        if prev_interval[1] >= interval[0] {
            // NOTE: overlaps
            *prev_interval = vec![prev_interval[0], std::cmp::max(interval[1], prev_interval[1])];
        } else {
            res.push(interval);
        }
    }
    res
}

pub fn main() {
    let intervals = vec![vec![1,3],vec![2,6],vec![8,10],vec![15,18]];
    assert_eq!(merge(intervals), vec![vec![1,6],vec![8,10],vec![15,18]]);
    let intervals = vec![vec![1,4],vec![4, 5]];
    assert_eq!(merge(intervals), vec![vec![1,5]]);
    let intervals = vec![vec![2,3],vec![4,5],vec![6,7],vec![8,9],vec![1,10]];
    assert_eq!(merge(intervals), vec![vec![1,10]]);
    print_pass(NAME)
}
