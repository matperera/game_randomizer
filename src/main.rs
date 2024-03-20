use clap::{Parser, Subcommand};
use colored::*;
use rand::prelude::*;
use rand_distr::WeightedIndex;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use std::thread::sleep;
use std::io;
mod rusqlite {
    pub mod rusqlite;
}


#[derive(Parser)]
#[command(name = "ref")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// search for a reference
    Reset,

    /// find and execute a reference
    Select,

    /// new setup
    New,

    /// allows to edit play date
    Edit,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        // example of inline definition
        Commands::Reset => {
            let _ = rusqlite::rusqlite::reset_counters();
        },

        Commands::Select => {
            randomize_data();
        },

        Commands::New => {
            let _ = rusqlite::rusqlite::create_table();
            let _ = rusqlite::rusqlite::fill_table();
        },

        Commands::Edit => {
            edit_consolse_counter();
        },
    }
}

pub fn randomize_data() {
    let table_exits = rusqlite::rusqlite::check_table().unwrap();
    if !table_exits {
        println!("{}", "Please create the table first using new.".yellow());
        return;
    }

    let console_data = rusqlite::rusqlite::get_consoles().unwrap();
    if console_data.is_empty() {
        println!("{}", "Please reset the counters using reset.".yellow());
        return;
    }

    let weights: Vec<i32> = console_data.iter().map(|x| x.plays_left * 5).collect();
    let weighted_index = WeightedIndex::new(&weights).unwrap();

    let mut rng = thread_rng();
    let random_console = console_data[weighted_index.sample(&mut rng)].console.as_str();
    let total = 50;
    let pb = ProgressBar::new(total);

    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .progress_chars("#>-"));

    for _ in 0..total {
        pb.inc(1);
        sleep(Duration::from_millis(50));
    }

    pb.finish_with_message("done");

    println!("Next I will be playing: {}!", random_console.bright_blue());

    let _ = rusqlite::rusqlite::update_one_counter(random_console);
}

pub fn edit_consolse_counter() {
    let mut console = String::new();
    println!("Enter the console you want to edit: ");
    io::stdin().read_line(&mut console).unwrap();
    let console = console.trim();

    let does_exists = rusqlite::rusqlite::find_one_console(console);
    if does_exists.is_err() {
        println!("{}", "Console not found! You can add using sqlite3".red());
        return;
    }

    let mut new_count = String::new();
    println!("Enter the new count: ");
    io::stdin().read_line(&mut new_count).unwrap();
    let new_count = new_count.trim();

    let _ = rusqlite::rusqlite::update_console_counter(console, new_count);
    println!("{}", "Counter updated successfully!".blue())
}