use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod analysis;

#[derive(Parser)]
#[command(name = "cli-test")]
#[command(about = "Analyzes dependencies in a code project")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Unused {
        path: PathBuf,
    },
    Circular {
        path: PathBuf,
    },
    Antipatterns {
        path: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Unused { path } => analysis::unused::run(&path),
        Commands::Circular { path } => analysis::circular::run(&path),
        Commands::Antipatterns { path } => analysis::antipatterns::run(&path),
    }
}