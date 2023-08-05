mod binary_search;
mod search_a_2d_matrix;
mod find_minimum_in_rotated_sorted_array;
mod search_in_rotated_sorted_array;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "Binary Search";

pub fn main() {
    print_module_name(MODULE_NAME);
    binary_search::main();
    search_a_2d_matrix::main();
    find_minimum_in_rotated_sorted_array::main();
    search_in_rotated_sorted_array::main();
}
