use colored::*;
use crate::rusqlite::rusqlite;

pub fn run() {
    let _ = rusqlite::reset_counters();
    println!("{}", "Counters reset successfully!".blue());
}
