use crate::cli::Cli;
use crate::commands::clean::clean_data;
use crate::commands::clone::clone_data;

pub fn build_database(cli: &Cli) {
    if !cli.no_clean {
        clean_data(cli);
    }
    
    clone_data(cli);
}