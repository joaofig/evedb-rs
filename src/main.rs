mod gitops;
mod cli;

use clap::Parser;
// use gitops::clone_repo;
use cli::Cli;
use crate::cli::Commands;


#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build => {
            println!("Building database...");
        }
        Commands::Clean => {
            println!("Cleaning data folder...");
        }
        Commands::Clone => {
            println!("Cloning source data...");
        }
    }
}
