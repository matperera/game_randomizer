use colored::*;
use clap::Parser;
use rand::prelude::*;
use rand_distr::WeightedIndex;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use std::thread::sleep;
use crate::rusqlite::rusqlite;

#[derive(Debug, Parser)]
pub struct Select {
    console_type: Option<String>,
}

pub fn run(select: &Select) {

    let console_data;
    match &select.console_type {
        Some(s) if s == "handheld" => {
            console_data = rusqlite::get_consoles(Some(1)).unwrap();
        },
        Some(s) if s == "console" => {
            console_data = rusqlite::get_consoles(Some(0)).unwrap();
        },
        Some(_) => {
            println!("{}", "Invalid argument. Please use either handheld or console.".red());
            return;
        },
        None => {
            console_data = rusqlite::get_consoles(None).unwrap();
        }
    }
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

    let _ = rusqlite::update_one_counter(random_console);
}
