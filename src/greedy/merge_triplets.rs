use crate::utils::print_pass;

const NAME: &str = "merge-triplets-to-form-target-triplet";


pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
    let mut res = vec![0; 3];
    let triplets: Vec<&Vec<i32>> = triplets.iter().filter(|triplet| {
        for i in 0..3 {
            if triplet[i] > target[i] {
                return false;
            }
        }
        return true;
    }).collect();
    for triplet in &triplets {
        for i in 0..3 {
            if triplet[i] == target[i] {
                res[i] = triplet[i];
            }
        }
    }
    for i in 0..3 {
        if res[i] != target[i] {
            return false;
        }
    }
    true
}

pub fn main() {
    let triplets = vec![vec![2,5,3],vec![1,8,4],vec![1,7,5]];
    let target = vec![2,7,5];
    assert_eq!(merge_triplets(triplets, target), true);
    let triplets = vec![vec![3,5,1],vec![10,5,7]];
    let target = vec![3,5,7];
    assert_eq!(merge_triplets(triplets, target), false);
    print_pass(NAME)
}
