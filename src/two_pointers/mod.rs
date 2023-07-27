mod is_palindrome;
mod two_sum_ii_array_is_sorted;
use crate::utils::print_module_name;
const MODULE_NAME: &str = "Two Pointers";

pub fn main() {
    print_module_name(MODULE_NAME);
    is_palindrome::main();
    two_sum_ii_array_is_sorted::main();
}
