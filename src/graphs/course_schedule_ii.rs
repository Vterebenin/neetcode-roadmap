use crate::utils::print_pass;

const NAME: &str = "course-schedule-ii";
const LINK: &str = "https://leetcode.com/problems/course-schedule-ii/";

const PREREQUISITE: usize = 0;
const COURSE: usize = 1;

#[derive(Copy, Clone)]
enum Status {
    Todo,
    InProgress,
    Done,
}

pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let num_courses = num_courses as usize;

    let mut graph = vec![Vec::new(); num_courses];
    for edge in prerequisites.iter() {
        graph[edge[PREREQUISITE] as usize].push(edge[COURSE] as usize);
    }

    let mut status = vec![Status::Todo; num_courses];
    let mut order = vec![];
    let is_valid = (0..num_courses).all(|course| !has_cycle(course, &mut status, &graph, &mut order));
    if !is_valid {
        return vec![];
    }
    order
}

fn has_cycle(course: usize, status: &mut Vec<Status>, graph: &Vec<Vec<usize>>, order: &mut Vec<i32>) -> bool {
    match status[course] {
        Status::Done => false,
        Status::InProgress => true,
        _ => {
            status[course] = Status::InProgress;
            if graph[course].iter().any(|&next_course| has_cycle(next_course, status, graph, order)) {
                return true;
            }
            order.push(course as i32);
            status[course] = Status::Done;
            false
        }
    }
}

pub fn main() {
    let num_courses = 4;
    let prerequisites = vec![vec![1,0],vec![2,0],vec![3,1],vec![3,2]];
    assert_eq!(find_order(num_courses, prerequisites), vec![0, 1, 2, 3]);
    let num_courses = 2;
    let prerequisites = vec![vec![1, 0]];
    assert_eq!(find_order(num_courses, prerequisites), vec![0, 1]);
    print_pass(NAME, LINK)
}
