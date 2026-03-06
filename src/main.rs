mod cli;
mod commands;
mod db;
mod etl;
mod models;
mod tools;

use crate::cli::Commands;
use crate::commands::build::build_database;
use crate::commands::builders::node::build_nodes;
use crate::commands::clean::clean_data;
use crate::commands::clone::clone_data;
use crate::commands::interactive::interactive;
use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() {
    let mut cli = Cli::parse();

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
        Commands::Interactive => {
            interactive(&mut cli).await;
        }
    }
}
