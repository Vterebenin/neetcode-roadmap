mod maximum_subarray;
mod jump_game;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "Greedy";

pub fn main() {
    print_module_name(MODULE_NAME);
    maximum_subarray::main();
    jump_game::main();
}
