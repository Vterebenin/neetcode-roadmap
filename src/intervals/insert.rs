use crate::utils::print_pass;

const NAME: &str = "insert-interval";


pub fn is_overlapping(i1: &Vec<i32>, i2: &Vec<i32>) -> bool {
    i1[0].max(i2[0]) <= i1[1].min(i2[1])
}
pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    if intervals.is_empty() {
        return vec![new_interval];
    }
    let mut min = new_interval[0];
    let mut max = new_interval[1];
    let mut copy = intervals.clone();
    let mut insert_in = intervals.len();
    let mut count_delete = 0;
    for (idx, interval) in intervals.iter().enumerate() {
        if is_overlapping(interval, &new_interval) {
            if insert_in == intervals.len() {
                insert_in = idx;
            }
            copy.remove(idx - count_delete);
            count_delete += 1;
            min = min.min(interval[0]).min(interval[1]);
            max = max.max(interval[0]).max(interval[1]);
        }
    }
    if copy.len() == intervals.len() {
        let new_insertion = vec![min, max];
        if intervals[intervals.len() - 1][1] < max {
            intervals.push(new_insertion);
        } else {
            // iterate and find second minor
            let mut insert_in = 0;
            for (idx, interval) in intervals.iter().enumerate() {
                if interval[0] > new_interval[1] {
                    if idx > 0 {
                        insert_in = idx;
                    }
                    break;
                }
            }
            intervals.insert(insert_in, vec![min, max]);
        }

        return intervals;
    }
    copy.insert(insert_in, vec![min, max]);
    copy
}

pub fn main() {
    let intervals = vec![vec![1, 3], vec![6,9]];
    let new_interval = vec![2, 5];
    assert_eq!(insert(intervals, new_interval), vec![vec![1, 5], vec![6, 9]]);
    let intervals = vec![vec![1,2],vec![3,5],vec![6,7],vec![8,10],vec![12,16]];
    let new_interval = vec![4, 8];
    assert_eq!(insert(intervals, new_interval), vec![vec![1,2],vec![3,10],vec![12,16]]);
    let intervals = vec![];
    let new_interval = vec![5,7];
    assert_eq!(insert(intervals, new_interval), vec![vec![5, 7]]);
    let intervals = vec![vec![3,5],vec![12,15]];
    let new_interval = vec![6, 6];
    assert_eq!(insert(intervals, new_interval), vec![vec![3, 5], vec![6, 6], vec![12, 15]]);
    print_pass(NAME)
}
