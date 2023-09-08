mod number_of_islands;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "Graphs";

pub fn main() {
    print_module_name(MODULE_NAME);
    number_of_islands::main();
}
