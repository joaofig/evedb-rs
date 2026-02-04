use clap::{Args, Parser, Subcommand};

#[derive(Parser, Clone, Debug)]
#[command(name = "MyApp")]
#[command(version = "0.1.0", author = "João Paulo (JP) Figueira")]
#[command(about = "Builds the eVED database from the original data sources")]
#[command(about, version, author)]
pub struct Cli {
    #[arg(long, default_value_t = String::from("./data/eved/repo"), help = "Sets the repositories path")]
    pub repo_path: String,

    #[arg(long, default_value_t = String::from("./data/eved/evedb.db"), help = "Sets the database path")]
    pub db_path: String,

    #[arg(long, help = "Verbose mode on")]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    /// builds the database
    #[command(about = "Builds the database")]
    Build(BuildCommandArgs),

    #[command(about = "Map-matches the trajectories")]
    Match,

    /// cleans the repositories folder
    #[command(about = "Cleans the repositories folder")]
    Clean,

    /// clones the source repositories
    #[command(about = "Clones the source repositories")]
    Clone,
}

#[derive(Args, Clone, Debug)]
pub struct BuildCommandArgs {
    #[arg(long, help = "Do not clone the repositories")]
    pub no_clone: bool,

    #[arg(long, help = "Do not clean the repositories folder after building")]
    pub no_clean: bool,
}
