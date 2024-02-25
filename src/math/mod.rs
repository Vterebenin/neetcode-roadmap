mod happy_number;
mod plus_one;
mod rotate_image;
mod spiral_matrix;
mod set_zeros;
mod multiply_strings;

use crate::utils::print_module_name;

const MODULE_NAME: &str = "Math";

pub fn main() {
    print_module_name(MODULE_NAME);
    happy_number::main();
    plus_one::main();
    rotate_image::main();
    spiral_matrix::main();
    set_zeros::main();
    multiply_strings::main();
}
