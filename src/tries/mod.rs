mod prefix_tree;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "Heap / Priority Queue";

pub fn main() {
    print_module_name(MODULE_NAME);
    prefix_tree::main();
}
