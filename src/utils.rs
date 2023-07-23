use colored::Colorize;
use colored::ColoredString;

pub fn print_pass(name: &str, link: &str) {
    let name_formatted: ColoredString = format!("✔️ {} passed! Solution for this problem: ", name).green();
    let link_formatted: ColoredString = format!("{}", link).blue().underline();
    let string_for_print: ColoredString = format!("  {}{}", name_formatted, link_formatted).on_black();
    println!("{}", string_for_print)
}

pub fn print_module_name(name: &str) {
    println!("{}", format!("{}:", name).bold().black().on_white().underline());
}
