mod number_of_islands;
mod max_area_island;
mod surrounded_regions;
mod rotten_oranges;
mod course_schedule;
mod redundant_connections;
mod course_schedule_ii;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "Graphs";

pub fn main() {
    print_module_name(MODULE_NAME);
    number_of_islands::main();
    max_area_island::main();
    surrounded_regions::main();
    rotten_oranges::main();
    course_schedule::main();
    redundant_connections::main();
    course_schedule_ii::main();
}
