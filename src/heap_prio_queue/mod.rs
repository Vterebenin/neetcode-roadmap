mod kth_largest;
mod stone_game;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "Heap / Priority Queue";

pub fn main() {
    print_module_name(MODULE_NAME);
    kth_largest::main();
    stone_game::main();
}
