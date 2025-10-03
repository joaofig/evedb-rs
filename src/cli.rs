// use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "MyApp")]
#[command(version = "0.1.0", author = "João Paulo (JP) Figueira")]
#[command(about = "Builds the eVED database from the original data sources")]
#[command(about, version, author)]
pub struct Cli {
    #[arg(long, default_value_t = String::from("./data"), help = "Sets the data path")]
    pub data_path: String,

    #[arg(long, help = "Verbose mode on")]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands
}


#[derive(Subcommand)]
pub enum Commands {
    /// builds the database
    Build,

    /// cleans the data folder
    Clean,

    /// clones the source data
    Clone,
}
