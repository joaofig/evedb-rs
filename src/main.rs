use clap::Parser;
use evedb::cli::{Cli, Commands};
use evedb::commands::build::build_database;
use evedb::commands::builders::node::build_nodes;
use evedb::commands::clean::clean_data;
use evedb::commands::clone::clone_data;
use evedb::commands::interactive::interactive;

#[tokio::main]
async fn main() {
    let mut cli = Cli::parse();

    match &cli.command {
        Some(Commands::Build(args)) => {
            build_database(&cli, args).await;
        }
        Some(Commands::Match) => {
            build_nodes(&cli).await;
        }
        Some(Commands::Clean) => {
            clean_data(&cli);
        }
        Some(Commands::Clone) => {
            clone_data(&cli);
        }
        Some(Commands::Interactive) => {
            interactive(&mut cli).await;
        }
        None => {
            interactive(&mut cli).await;
        }
    }
}
