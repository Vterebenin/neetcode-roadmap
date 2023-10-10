mod happy_number;
mod plus_one;
mod rotate_image;

use crate::utils::print_module_name;

const MODULE_NAME: &str = "Math";

pub fn main() {
    print_module_name(MODULE_NAME);
    happy_number::main();
    plus_one::main();
    rotate_image::main();
}
