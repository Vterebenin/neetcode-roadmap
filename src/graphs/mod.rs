mod number_of_islands;
mod max_area_island;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "Graphs";

pub fn main() {
    print_module_name(MODULE_NAME);
    number_of_islands::main();
    max_area_island::main();
}
