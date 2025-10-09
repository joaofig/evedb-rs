use std::fs;
use std::process::Command;
use std::path::Path;
use crate::cli::Cli;

fn clone_repo(cli: &Cli, clone_url: &str, destination: &str,) -> bool {
    if Path::new(destination).exists() {
        if cli.verbose {
            println!("Repository already exists at {}", destination);
        }
        return true;
    }

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
    output.status.success()
}

pub fn clone_data(cli: &Cli) -> bool {
    let eved_destination: String = cli.repo_path.clone() + "/eved";
    let ved_destination: String = cli.repo_path.clone() + "/ved";

    if fs::remove_dir_all(cli.repo_path.clone()).is_ok() {
        if cli.verbose {
            println!("Removed existing repository at {}", cli.repo_path);
        }
    }

    if clone_repo(cli, "https://bitbucket.org/datarepo/eved_dataset.git",
               &eved_destination) {
        clone_repo(cli, "https://github.com/gsoh/VED.git",
                   &ved_destination)
    } else {
        false
    }
}
