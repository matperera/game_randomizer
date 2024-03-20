use clap::Parser;
use colored::*;
use crate::rusqlite::rusqlite;
use std::io;

#[derive(Debug, Parser)]
pub struct Edit {
    console: Option<String>,
}

pub fn run(edit: &Edit) {
    match &edit.console {
        Some(console) => {
            let does_exists = rusqlite::find_one_console(console);
            if does_exists.is_err() {
                println!("{}", "Console not found! You can add using sqlite3".red());
                return;
            }

            run_edit(console);
        },
        None => {
            let mut console = String::new();
            println!("Enter the console you want to edit: ");
            io::stdin().read_line(&mut console).unwrap();
            let console = console.trim();

            let does_exists = rusqlite::find_one_console(console);
            if does_exists.is_err() {
                println!("{}", "Console not found! You can add using sqlite3".red());
                return;
            }

            run_edit(console);
        }
    }
}

pub fn run_edit(console: &str) {
    let mut new_count = String::new();
    println!("Enter the new count: ");
    io::stdin().read_line(&mut new_count).unwrap();
    let new_count = new_count.trim();

    let _ = rusqlite::update_console_counter(console, new_count);
    println!("{}", "Counter updated successfully!".blue());
}