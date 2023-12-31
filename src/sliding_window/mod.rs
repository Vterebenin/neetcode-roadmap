mod max_profit;
mod longest_substring;
mod string_permutation;
mod longest_repeating;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "Sliding Window";

pub fn main() {
    print_module_name(MODULE_NAME);
    max_profit::main();
    longest_substring::main();
    longest_repeating::main();
    string_permutation::main();
}
