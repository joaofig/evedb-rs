// use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "MyApp")]
#[command(version = "0.1.0", author = "João Paulo (JP) Figueira")]
#[command(about = "Builds the eVED database from the original data sources")]
#[command(about, version, author)]
pub struct Cli {
    #[arg(long, default_value_t = String::from("~/data/eved/repo"), help = "Sets the repositories path")]
    pub repo_path: String,

    #[arg(long, help = "Do not clean the repositories folder prior to cloning")]
    pub no_clean: bool,

    #[arg(long, help = "Do not clone the repositories")]
    pub no_clone: bool,

    #[arg(long, help = "Verbose mode on")]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands
}


#[derive(Subcommand)]
pub enum Commands {
    /// builds the database
    #[command(about = "Builds the database")]
    Build,

    /// cleans the repositories folder
    #[command(about = "Cleans the repositories folder")]
    Clean,

    /// clones the source repositories
    #[command(about = "Clones the source repositories")]
    Clone,
}
