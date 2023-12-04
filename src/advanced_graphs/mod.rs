pub mod network_delay_time;
pub mod reconstruct_itinerary;
pub mod min_cost_to_connect_all_points;
pub mod cheapest_flights_within_k_stops;

use crate::utils::print_module_name;
const MODULE_NAME: &str = "Advanced Graphs";

pub fn main() {
    print_module_name(MODULE_NAME);
    network_delay_time::main();
    reconstruct_itinerary::main();
    min_cost_to_connect_all_points::main();
    cheapest_flights_within_k_stops::main();
}
