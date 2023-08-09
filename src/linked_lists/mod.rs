mod reverse_linked_list;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "Linked Lists";

pub fn main() {
    print_module_name(MODULE_NAME);
    reverse_linked_list::main();
}
