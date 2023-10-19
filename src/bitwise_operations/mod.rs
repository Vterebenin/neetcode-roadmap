mod reverse_bits;
mod number_of_one_bits;
mod single_number;
mod missing_number;
mod counting_bits;
mod get_sum;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "Bitwise operations";

pub fn main() {
    print_module_name(MODULE_NAME);
    reverse_bits::main();
    number_of_one_bits::main();
    single_number::main();
    missing_number::main();
    counting_bits::main();
    get_sum::main();
}
