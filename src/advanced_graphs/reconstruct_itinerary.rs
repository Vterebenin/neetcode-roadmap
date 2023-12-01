use crate::utils::print_pass;

use std::collections::{HashMap, VecDeque};

pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for ticket in tickets {
        let from = &ticket[0];
        let to = &ticket[1];
        graph.entry(from.clone()).and_modify(|e| e.push(to.clone())).or_insert(vec![to.clone()]);
    }
    for to_arr in graph.values_mut() {
        to_arr.sort();
        to_arr.reverse();
    }

    fn dfs(graph: &mut HashMap<String, Vec<String>>, airport: &String, itinerary: &mut Vec<String>) {
        while let Some(next) = graph.get_mut(airport).and_then(|dests| dests.pop()) {
            dfs(graph, &next, itinerary);
        }
        itinerary.push(airport.to_string());
    }

    let mut itinerary: Vec<String> = vec![];
    dfs(&mut graph, &"JFK".to_string(), &mut itinerary);

    itinerary.reverse();
    itinerary
}
const NAME: &str = "reconstruct-itinerary";

pub fn main() {
    let tickets = vec![
        vec!["MUC".to_string(), "LHR".to_string()],
        vec!["JFK".to_string(), "MUC".to_string()],
        vec!["SFO".to_string(), "SJC".to_string()],
        vec!["LHR".to_string(), "SFO".to_string()],
    ];
    assert_eq!(
        find_itinerary(tickets),
        vec![
            "JFK".to_string(),
            "MUC".to_string(),
            "LHR".to_string(),
            "SFO".to_string(),
            "SJC".to_string()
        ]
    );

    print_pass(NAME)
}
