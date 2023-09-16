mod climbing_staircase;
mod robber_one;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "1D - dynamic programming";

pub fn main() {
    print_module_name(MODULE_NAME);
    climbing_staircase::main();
    robber_one::main();
}
