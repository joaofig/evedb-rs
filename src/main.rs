mod cli;
mod commands;
mod db;
mod etl;
mod models;
mod tools;

use crate::cli::Commands;
use crate::commands::build::build_database;
use crate::commands::clean::clean_data;
use crate::commands::clone::clone_data;
use clap::Parser;
use cli::Cli;
use crate::commands::builders::node::build_nodes;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build(args) => {
            build_database(&cli, args).await;
        }
        Commands::Match => {
            build_nodes(&cli).await;
        }
        Commands::Clean => {
            clean_data(&cli);
        }
        Commands::Clone => {
            clone_data(&cli);
        }
    }
}
