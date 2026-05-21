mod models;
mod platforms;
mod scanner;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Scan {},
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Scan {} => {
            let a = scanner::scanner::scan();
            println!("Found {} files", a.len());
        }
    }
}
