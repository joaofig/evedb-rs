use crate::cli::Cli;
use std::fs;
use std::path::Path;
use std::process::Command;

fn clone_repo(cli: &Cli, clone_url: &str, destination: &str) -> bool {
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
        eprintln!(
            "Error cloning repository: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    output.status.success()
}

pub fn clone_data(cli: &Cli) -> bool {
    let eved_path: String = cli.repo_path.clone() + "/eved";
    let ved_path: String = cli.repo_path.clone() + "/ved";

    if fs::remove_dir_all(cli.repo_path.clone()).is_ok() && cli.verbose {
        println!("Removed existing repository data at {}", cli.repo_path);
    }
    clone_repo(
        cli,
        "https://bitbucket.org/datarepo/eved_dataset.git",
        &eved_path,
    ) && clone_repo(cli, "https://github.com/gsoh/VED.git", &ved_path)
}


fn print_file_exists(file_name: &str) {
    if Path::new(file_name).exists()  {
        print!("✅ {}", file_name);
    } else {
        print!("❌ {}", file_name);
    }
}


pub fn display_status(cli: &Cli) {
    let eved_path: String = cli.repo_path.clone() + "/eved";
    let ved_path: String = cli.repo_path.clone() + "/ved";

    if Path::new(&ved_path).exists() {
        println!("✅ {}", ved_path);

        let xlsx1 = ved_path.clone() + "/Data/VED_Static_Data_ICE&HEV.xlsx";
        print_file_exists(xlsx1.as_str());

        let xlsx2 = ved_path.clone() + "/Data/VED_Static_Data_PHEV&EV.xlsx";
        print_file_exists(xlsx2.as_str());
    } else {
        println!("❌ {}", ved_path);
    }

    if Path::new(&eved_path).exists() {
        println!("✅ {}", eved_path);

        let zip = eved_path.clone() + "/data/eved.zip";
        print_file_exists(&zip);
    } else {
        println!("❌ {}", eved_path);
    }
}