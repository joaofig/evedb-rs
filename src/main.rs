mod gitops;

use std::path::PathBuf;
use clap::Parser;
use gitops::clone_repo;

#[derive(Parser)]
#[command(name = "MyApp")]
#[command(version = "0.1.0", author = "João Paulo (JP) Figueira")]
#[command(about = "Builds the eVED database from the original data sources")]
#[command(about, version, author)]
struct Cli {
    #[arg(short, long, help = "Checks if the eVED repository is up to date")]
    eved: bool,

    #[arg(short, long, help = "Clones the VED repository from GitHub")]
    ved: bool,

    #[arg(short, long, help = "Builds the signal table from the original data sources")]
    signals: bool,

    #[arg(short, long, help = "Builds the nodes table from the original data sources")]
    nodes: bool,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>
}

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
}
