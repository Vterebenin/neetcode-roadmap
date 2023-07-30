mod valid_parentheses;
mod min_stack;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "Stack";

pub fn main() {
    print_module_name(MODULE_NAME);
    valid_parentheses::main();
    min_stack::main();
}
