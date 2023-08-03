mod binary_search;
mod search_a_2d_matrix;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "Binary Search";

pub fn main() {
    print_module_name(MODULE_NAME);
    binary_search::main();
    search_a_2d_matrix::main();
}
