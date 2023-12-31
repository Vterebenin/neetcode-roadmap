mod maximum_subarray;
mod jump_game;
mod jump_game_two;
mod merge_triplets;
mod hand_of_straight;
mod valid_parenthesis_string;
mod partition_labels;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "Greedy";

pub fn main() {
    print_module_name(MODULE_NAME);
    maximum_subarray::main();
    jump_game::main();
    jump_game_two::main();
    merge_triplets::main();
    hand_of_straight::main();
    valid_parenthesis_string::main();
    partition_labels::main();
}
