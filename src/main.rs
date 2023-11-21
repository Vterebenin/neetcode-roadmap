mod arrays_and_hashing;
mod two_pointers;
mod stack;
mod binary_search;
mod sliding_window;
mod linked_lists;
mod utils;
mod trees;
mod backtracking;
mod heap_prio_queue;
mod tries;
mod graphs;
mod math;
mod lru;
mod dynamic_one;
mod bitwise_operations;
mod intervals;
mod greedy;
mod advanced_graphs;


use utils::print_total;

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
    heap_prio_queue::main();
    tries::main();
    graphs::main();
    math::main();
    lru::main();
    dynamic_one::main();
    bitwise_operations::main();
    intervals::main();
    greedy::main();
    advanced_graphs::main();
    print_total();
}
