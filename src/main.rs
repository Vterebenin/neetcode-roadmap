mod arrays_and_hashing;
mod two_pointers;
mod stack;
mod binary_search;
mod sliding_window;
mod linked_lists;
mod utils;
mod trees;
mod backtracking;


use crate::utils::clear_terminal;

fn main() {
    clear_terminal();
    arrays_and_hashing::main();
    two_pointers::main();
    stack::main();
    binary_search::main();
    sliding_window::main();
    linked_lists::main();
    trees::main();
    backtracking::main();
}
