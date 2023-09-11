use crate::utils::print_pass;

const NAME: &str = "course-schedule";
const LINK: &str = "https://leetcode.com/problems/course-schedule/";

const PREREQUISITE: usize = 0;
const COURSE: usize = 1;

#[derive(Copy, Clone)]
enum Status {
    Todo,
    InProgress,
    Done,
}

pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let num_courses = num_courses as usize; // Thanks Leetcode for your sh*tty method signatures in Rust.

    let mut graph = vec![Vec::new(); num_courses]; // Dependencies graph between courses.
    for edge in prerequisites.iter() {
        graph[edge[PREREQUISITE] as usize].push(edge[COURSE] as usize);
    }

    let mut status = vec![Status::Todo; num_courses];
    (0..num_courses).all(|course| !has_cycle(course, &mut status, &graph))
}

fn has_cycle(course: usize, status: &mut Vec<Status>, graph: &Vec<Vec<usize>>) -> bool {
    match status[course] {
        Status::Done => false,
        Status::InProgress => true,
        _ => {
            status[course] = Status::InProgress;
            if graph[course].iter().any(|&next_course| has_cycle(next_course, status, graph)) {
                return true;
            }
            status[course] = Status::Done;
            false
        }
    }
}

pub fn main() {
    let num_courses = 2;
    let prerequisites = vec![vec![1, 0]];
    assert_eq!(can_finish(num_courses, prerequisites), true);
    print_pass(NAME, LINK)
}
