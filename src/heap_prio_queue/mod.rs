mod kth_largest;
mod stone_game;
mod k_closest_to_origin;
mod twitter;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "Heap / Priority Queue";

pub fn main() {
    print_module_name(MODULE_NAME);
    kth_largest::main();
    stone_game::main();
    k_closest_to_origin::main();
    twitter::main();
}
