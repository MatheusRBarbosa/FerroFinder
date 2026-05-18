mod models;
mod scanner;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Scan {
        path: String,
    }
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Scan { path } => {
            let a = scanner::scanner::scan_directory(&path);
            print!("Found {} files", a.len());
        }
    }
}
