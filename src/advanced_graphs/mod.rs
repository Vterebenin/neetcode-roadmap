pub mod network_delay_time;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "Advanced Graphs";

pub fn main() {
    print_module_name(MODULE_NAME);
    network_delay_time::main();
}