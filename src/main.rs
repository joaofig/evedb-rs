mod gitops;
mod cli;

use clap::Parser;
use gitops::clone_repo;
use cli::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // Clone the eVED repository
    if cli.eved {
        clone_repo("https://bitbucket.org/datarepo/eved_dataset.git",
        "eved");
    }

    // Clone the VED repository
    if cli.ved {
        clone_repo("https://github.com/gsoh/VED.git",
        "ved");
    }

    if (cli.verbose) {
        println!("Data path: {}", cli.data_path.unwrap_or_else(|| "./data".to_string()));
    }
}
