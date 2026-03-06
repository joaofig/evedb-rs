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
