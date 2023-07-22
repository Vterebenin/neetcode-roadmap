use colored::Colorize;
pub mod check_duplicate;

pub fn main() {

    // println!(
    //     "{}, {}, {}, {}, {}, {}, and some normal text.",
    //     "Bold".bold(),
    //     "Red".red(),
    //     "Yellow".yellow(),
    //     "Green Strikethrough".green().strikethrough(),
    //     "Blue Underline".blue().underline(),
    //     "Purple Italics".purple().italic()
    //     );
    println!("{}", "Arrays and hashing:".bold().blue().underline());
    check_duplicate::main();
}
