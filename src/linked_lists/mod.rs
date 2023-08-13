mod reverse_linked_list;
mod merge_two_sorted_lists;
mod remove_nth_node;
mod reorder;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "Linked Lists";

pub fn main() {
    print_module_name(MODULE_NAME);
    reverse_linked_list::main();
    merge_two_sorted_lists::main();
    remove_nth_node::main();
    reorder::main();
}
