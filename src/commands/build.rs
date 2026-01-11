use crate::cli::{BuildCommandArgs, Cli};
use crate::commands::clean::clean_data;
use crate::commands::clone::clone_data;
use crate::commands::builders::trajectory::build_trajectories;
use crate::commands::builders::vehicle::build_vehicles;
use crate::commands::builders::signal::build_signals;

pub async fn build_database(cli: &Cli, args: &BuildCommandArgs) {
    if !args.no_clone {
        clone_data(cli);
    }

    build_vehicles(cli).await;
    build_signals(cli).await;
    build_trajectories(cli).await;

    if !args.no_clean {
        clean_data(cli);
    }
}
