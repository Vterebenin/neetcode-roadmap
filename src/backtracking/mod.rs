mod subsets;
mod permutations;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "Backtracking";

pub fn main() {
    print_module_name(MODULE_NAME);
    subsets::main();
    permutations::main();
}
