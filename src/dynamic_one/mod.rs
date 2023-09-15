mod climbing_staircase;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "1D - dynamic programming";

pub fn main() {
    print_module_name(MODULE_NAME);
    climbing_staircase::main();
}
