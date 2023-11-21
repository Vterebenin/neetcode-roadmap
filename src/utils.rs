use std::sync::atomic::{AtomicUsize, Ordering};
use colored::Colorize;
use colored::ColoredString;

static TOTAL_CALLS: AtomicUsize = AtomicUsize::new(0);

pub fn print_pass(name: &str) {
    let link = format!("https://leetcode.com/problems/{}/", name);
    let name_formatted: ColoredString = format!("✔️ {} passed! Solution for this problem: ", name).green();
    let link_formatted: ColoredString = format!("{}", link).blue().underline();
    let string_for_print: ColoredString = format!("  {}{}", name_formatted, link_formatted).on_black();
    TOTAL_CALLS.fetch_add(1, Ordering::SeqCst);
    println!("{}", string_for_print)
}

pub fn print_module_name(name: &str) {
    println!("{}", format!("{}:", name).bold().black().on_white().underline());
}

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}
pub fn print_total() {
    println!("________________________________________________________________________________");
    let total = TOTAL_CALLS.load(Ordering::SeqCst);
    let total = format!("Overall solutions passed: {}", total);
    println!("{}", format!("{}", total).cyan().on_magenta().underline());
}
