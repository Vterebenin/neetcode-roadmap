use crate::utils::print_pass;

const NAME: &str = "daily-temperatures";


pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let n = temperatures.len();
    let mut result = vec![0; n];
    let mut stack = vec![];
    for (i, _) in temperatures.iter().enumerate() {
        while stack.last().map_or(false, |&j| temperatures[j] < temperatures[i]) {
            stack.pop().map(|j| {
                result[j] = (i - j) as i32;
            });
        }
        stack.push(i);
    }
    result
}

pub fn main() {
    let temps: Vec<i32> = vec![73,74,75,71,69,72,76,73];
    assert_eq!(daily_temperatures(temps), [1,1,4,2,1,1,0,0]);
    print_pass(NAME)
}
