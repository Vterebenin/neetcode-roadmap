mod climbing_staircase;
mod robber_one;
mod min_cost_climbing_stairs;
mod decode_ways;
mod unique_paths;
mod find_target_sum_ways;
mod rob;
mod longest_palindromic_substring;
mod coin_change;
mod longest_increasing_subsequence;
mod partition_equal_subset_sum;
mod count_substrings;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "1D - dynamic programming";

pub fn main() {
    print_module_name(MODULE_NAME);
    climbing_staircase::main();
    robber_one::main();
    min_cost_climbing_stairs::main();
    decode_ways::main();
    unique_paths::main();
    find_target_sum_ways::main();
    rob::main();
    longest_palindromic_substring::main();
    coin_change::main();
    longest_increasing_subsequence::main();
    partition_equal_subset_sum::main();
    count_substrings::main();
}
