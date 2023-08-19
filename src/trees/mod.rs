mod invert_binary_tree;
mod max_depth;
mod diametr;
mod balanced;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "Trees";

pub fn main() {
    print_module_name(MODULE_NAME);
    invert_binary_tree::main();
    max_depth::main();
    diametr::main();
    balanced::main();
}
