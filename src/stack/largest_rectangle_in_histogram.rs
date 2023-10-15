use crate::utils::print_pass;

const NAME: &str = "valid-parentheses";
const LINK: &str = "https://leetcode.com/problems/valid-parentheses/";

pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut stack = Vec::new();
    let mut max_area = 0;

    for (i, &height) in heights.iter().chain(&[0]).enumerate() {
        while let Some(&top) = stack.last() {
            if height < heights[top] {
                let top_height = heights[stack.pop().unwrap()];
                let width = match stack.last() {
                    Some(&prev) => (i - prev - 1) as i32,
                    None => i as i32,
                };
                max_area = max_area.max(top_height * width);
            } else {
                break;
            }
        }
        stack.push(i);
    }

    max_area
}

pub fn main() {
    let heights = vec![2, 1, 5, 6, 2, 3];
    assert_eq!(largest_rectangle_area(heights), 10);
    let heights = vec![2, 4];
    assert_eq!(largest_rectangle_area(heights), 4);
    let heights = vec![1, 1];
    assert_eq!(largest_rectangle_area(heights), 2);
    let heights = vec![0, 9];
    assert_eq!(largest_rectangle_area(heights), 9);
    print_pass(NAME, LINK)
}
