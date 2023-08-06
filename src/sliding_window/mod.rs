mod max_profit;
mod longest_substring;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "Sliding Window";

pub fn main() {
    print_module_name(MODULE_NAME);
    max_profit::main();
    longest_substring::main();
}
