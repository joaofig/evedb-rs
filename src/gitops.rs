
use std::process::Command;

fn rm_destination(destination: &str) -> bool {
    // Remove the destination folder if it exists
    let mut rm_cmd = Command::new("rm");
    rm_cmd.args(["-rf", destination]);

    // Execute the clone command
    let output = rm_cmd.output().expect("Failed to remove destination folder");

    output.status.success()
}

pub fn clone_repo(clone_url: &str, destination: &str,) {
    if rm_destination(destination) {
        // Prepare git clone command
        let mut cmd = Command::new("git");
        cmd.args(["clone", clone_url]);
        cmd.arg(destination);

        println!("Cloning repository from {}", clone_url);

        // Execute the clone command
        let output = cmd.output().expect("Failed to execute git clone");

        if output.status.success() {
            println!("Repository cloned successfully to {}", destination);
        } else {
            eprintln!("Error cloning repository:");
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }
    }
}