mod cli;
mod commands;

use clap::Parser;
// use gitops::clone_repo;
use cli::Cli;
use crate::cli::Commands;
use crate::commands::clean::clean_data;
use crate::commands::clone::clone_data;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build => {
            println!("Building database...");
        }
        Commands::Clean => {
            clean_data(&cli);
        }
        Commands::Clone => {
            clone_data(&cli);
        }
    }
}
