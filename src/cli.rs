use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(name = "MyApp")]
#[command(version = "0.1.0", author = "João Paulo (JP) Figueira")]
#[command(about = "Builds the eVED database from the original data sources")]
#[command(about, version, author)]
pub struct Cli {
    #[arg(long, help = "Clones the eVED repository from GitHub")]
    pub eved: bool,

    #[arg(long, help = "Clones the VED repository from GitHub")]
    pub ved: bool,

    #[arg(short, long, help = "Enables verbose mode")]
    pub verbose: bool,

    #[arg(short, long, help = "Builds the signal table from the original data sources")]
    pub signals: bool,

    #[arg(short, long, help = "Builds the nodes table from the original data sources")]
    pub nodes: bool,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[arg(long, value_name = "./data", help = "Sets the data path")]
    pub data_path: Option<String>,
}
