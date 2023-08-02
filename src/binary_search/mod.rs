mod binary_search;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "Binary Search";

pub fn main() {
    print_module_name(MODULE_NAME);
    binary_search::main();
}
