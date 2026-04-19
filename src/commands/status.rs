use crate::cli::Cli;
use crate::commands::clone;

pub fn display_status(cli: &Cli) {
    println!("\nStatus:");

    clone::display_status(cli)
}