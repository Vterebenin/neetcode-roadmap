mod lru_cache;

use crate::utils::print_module_name;

const MODULE_NAME: &str = "LRU";

pub fn main() {
    print_module_name(MODULE_NAME);
    lru_cache::main();
}
