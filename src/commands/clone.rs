use std::process::Command;
use crate::cli::Cli;
use crate::commands::clean::clean_data;

fn clone_repo(cli: &Cli, clone_url: &str, destination: &str,) {
    clean_data(cli);

    // Prepare git clone command
    let mut cmd = Command::new("git");
    cmd.args(["clone", clone_url]);
    cmd.arg(destination);

    if cli.verbose {
        println!("Cloning a repository from {}", clone_url);
    }

    // Execute the clone command
    let output = cmd.output().expect("Failed to execute git clone");

    if output.status.success() {
        if cli.verbose {
            println!("Repository cloned successfully to {}", destination);
        }
    } else {
        eprintln!("Error cloning repository:");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
}

pub fn clone_data(cli: &Cli) {
    let eved_destination: String = cli.data_path.clone() + "/eved";
    let ved_destination: String = cli.data_path.clone() + "/ved";

    clone_repo(cli, "https://bitbucket.org/datarepo/eved_dataset.git", &eved_destination);

    clone_repo(cli, "https://github.com/gsoh/VED.git", &ved_destination);
}
