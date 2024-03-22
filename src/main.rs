use clap::{Parser, Subcommand};
mod rusqlite {
    pub mod rusqlite;
}
mod command;


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
    Select(command::select::Select),

    /// allows to edit play date
    Edit(command::edit::Edit),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = rusqlite::rusqlite::setup()?;

    let cli = Cli::parse();

    match &cli.command {
        // example of inline definition
        Commands::Reset => {
            command::reset::run();
        },

        Commands::Select(select) => {
            command::select::run(&select);
        },

        Commands::Edit(edit) => {
            command::edit::run(&edit);
        },
    }

    Ok(())
}
