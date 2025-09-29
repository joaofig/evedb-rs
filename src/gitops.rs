
use std::process::Command;
use std::fs;

fn rm_destination(destination: &str) -> bool {
    fs::remove_dir_all(destination).is_ok()
}

pub fn clone_repo(clone_url: &str, destination: &str,) {
    rm_destination(destination);

    // Prepare git clone command
    let mut cmd = Command::new("git");
    cmd.args(["clone", clone_url]);
    cmd.arg(destination);

    println!("Cloning a repository from {}", clone_url);

    // Execute the clone command
    let output = cmd.output().expect("Failed to execute git clone");

    if output.status.success() {
        println!("Repository cloned successfully to {}", destination);
    } else {
        eprintln!("Error cloning repository:");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
}