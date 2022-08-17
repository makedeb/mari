use crate::style::Colorize;

pub fn info(str: &str) {
    println!("{} {}", "Info:".cyan().bold(), str);
}

pub fn warning(str: &str) {
    println!("{} {}", "Warning:".yellow().bold(), str);
}

pub fn error(str: &str) {
    println!("{} {}", "Err:".red().bold(), str);
}
