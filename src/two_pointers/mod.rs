mod is_palindrome;
mod two_sum_ii_array_is_sorted;
mod three_sum;
mod container_with_most_water;
use crate::utils::print_module_name;
const MODULE_NAME: &str = "Two Pointers";

pub fn main() {
    print_module_name(MODULE_NAME);
    is_palindrome::main();
    two_sum_ii_array_is_sorted::main();
    three_sum::main();
    container_with_most_water::main();
}
