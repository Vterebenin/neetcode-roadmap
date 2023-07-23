use colored::Colorize;
pub mod check_duplicate;
pub mod valid_anagram;

pub fn main() {
    println!("{}", "Arrays and hashing:".bold().blue().underline());
    check_duplicate::main();
    valid_anagram::main();
}
