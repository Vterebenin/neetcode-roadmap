mod climbing_staircase;
mod robber_one;
mod min_cost_climbing_stairs;
mod decode_ways;
mod unique_paths;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "1D - dynamic programming";

pub fn main() {
    print_module_name(MODULE_NAME);
    climbing_staircase::main();
    robber_one::main();
    min_cost_climbing_stairs::main();
    decode_ways::main();
    unique_paths::main();
}
