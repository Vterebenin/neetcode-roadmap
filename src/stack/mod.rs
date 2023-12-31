mod valid_parentheses;
mod min_stack;
mod polish_notation;
mod daily_temperatures;
mod largest_rectangle_in_histogram;
mod car_fleet;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "Stack";

pub fn main() {
    print_module_name(MODULE_NAME);
    valid_parentheses::main();
    min_stack::main();
    polish_notation::main();
    daily_temperatures::main();
    largest_rectangle_in_histogram::main();
    car_fleet::main();
}
