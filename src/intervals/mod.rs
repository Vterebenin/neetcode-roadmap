mod insert;
mod merge;
mod non_overlapping_intervals;

use crate::utils::print_module_name;

const MODULE_NAME: &str = "Intervals";

pub fn main() {
    print_module_name(MODULE_NAME);
    insert::main();
    merge::main();
    non_overlapping_intervals::main();
}
