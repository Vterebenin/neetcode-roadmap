mod insert;
mod merge;

use crate::utils::print_module_name;

const MODULE_NAME: &str = "Intervals";

pub fn main() {
    print_module_name(MODULE_NAME);
    insert::main();
    merge::main();
}
