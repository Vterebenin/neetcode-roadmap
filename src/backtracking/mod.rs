mod subsets;
mod permutations;
mod subsets2;
mod word_search;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "Backtracking";

pub fn main() {
    print_module_name(MODULE_NAME);
    subsets::main();
    permutations::main();
    subsets2::main();
    word_search::main();
}
