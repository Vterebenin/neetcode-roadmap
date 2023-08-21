mod invert_binary_tree;
mod max_depth;
mod diametr;
mod balanced;
mod is_same_tree;
mod lca_binary_tree;
mod subtree;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "Trees";

pub fn main() {
    print_module_name(MODULE_NAME);
    invert_binary_tree::main();
    max_depth::main();
    diametr::main();
    balanced::main();
    is_same_tree::main();
    lca_binary_tree::main();
    subtree::main();
}
