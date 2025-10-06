use crate::cli::{BuildCommandArgs, Cli};
use crate::commands::clean::clean_data;
use crate::commands::clone::clone_data;

pub fn build_database(cli: &Cli, args: &BuildCommandArgs) {
    if !args.no_clean {
        clean_data(cli);
    }

    if !args.no_clone {
        clone_data(cli);
    }
}