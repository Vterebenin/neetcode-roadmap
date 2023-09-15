mod reverse_bits;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "Bitwise operations";

pub fn main() {
    print_module_name(MODULE_NAME);
    reverse_bits::main();
}
