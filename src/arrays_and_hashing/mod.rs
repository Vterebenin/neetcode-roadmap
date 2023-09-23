pub mod check_duplicate;
pub mod valid_anagram;
pub mod two_sum;
pub mod group_anagram;
pub mod top_k_frequent_elements;
pub mod product_of_array_except_self;
mod longest_consecutive_sequence;
mod valid_sudoku;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "Arrays and hashing";

pub fn main() {
    print_module_name(MODULE_NAME);
    check_duplicate::main();
    valid_anagram::main();
    two_sum::main();
    group_anagram::main();
    top_k_frequent_elements::main();
    product_of_array_except_self::main();
    longest_consecutive_sequence::main();
    valid_sudoku::main();
}
