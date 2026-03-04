use crate::cli::Cli;
use inquire::{error::InquireError, Select};


fn get_menu_option() -> String {
    loop {
        let options: Vec<&str> = vec![
            "Database",
            "Repository",
            "Clean",
            "Clone",
            "Build",
            "Match",
            "Exit",
        ];
        let ans: Result<&str, InquireError> = Select::new("Please select an option:", options).prompt();
        if let Ok(option) = ans {
            return option.to_string();
        }
    }
}

pub fn interactive(cli: &Cli) {
    let mut option = "".to_string();

    while option != "Exit" {
        option = get_menu_option();
        match option.as_str() {
            "Database" => {
                println!("Database");
            },
            "Repository" => {
                println!("Repository");
            },
            "Clean" => {
                println!("Clean");
            },
            "Clone" => {
                println!("Clone");
            },
            &_ => {
                eprintln!("Invalid option");
            }
        }
    }

    if cli.verbose {
        println!("Exiting...")
    }
}
