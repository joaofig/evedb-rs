use crate::cli::Cli;
use std::fs;

pub fn clean_data(cli: &Cli) -> bool {
    if cli.verbose {
        println!("Cleaning repositories folder...");
    }
    fs::remove_dir_all(cli.repo_path.clone()).is_ok()
}
