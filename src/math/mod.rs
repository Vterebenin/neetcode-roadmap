mod happy_number;

use crate::utils::print_module_name;

const MODULE_NAME: &str = "Math";

pub fn main() {
    print_module_name(MODULE_NAME);
    happy_number::main();
}
