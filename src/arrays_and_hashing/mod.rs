pub mod check_duplicate;
pub mod valid_anagram;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "Arrays and hashing";
pub fn main() {
    print_module_name(MODULE_NAME);
    check_duplicate::main();
    valid_anagram::main();
}
