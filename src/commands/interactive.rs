use crate::cli::{BuildCommandArgs, Cli};
use crate::commands::build::build_database;
use crate::commands::builders::node::build_nodes;
use crate::commands::clean::clean_data;
use crate::commands::clone::clone_data;
use inquire::{Select, Text, error::InquireError};

fn get_menu_option() -> String {
    loop {
        let options: Vec<&str> = vec![
            "database",
            "repository",
            "clean",
            "clone",
            "build",
            "match",
            "exit",
        ];
        let ans: Result<&str, InquireError> =
            Select::new("Please select an option:", options).prompt();
        if let Ok(option) = ans {
            return option.to_string();
        }
    }
}

pub async fn interactive(cli: &mut Cli) {
    let mut option = "".to_string();

    // Force verbose mode in interactive sessions
    cli.verbose = true;

    while option != "exit" {
        println!("repository: {}", cli.repo_path);
        println!("database  : {}", cli.db_path);
        println!();

        option = get_menu_option();
        match option.as_str() {
            "database" => {
                let database = Text::new("Database path")
                    .with_default(&cli.db_path)
                    .prompt();
                if let Ok(database) = database {
                    cli.db_path = database;
                }
            }
            "repository" => {
                let repository = Text::new("Repository path")
                    .with_default(&cli.repo_path)
                    .prompt();
                if let Ok(repository) = repository {
                    cli.repo_path = repository;
                }
            }
            "clean" => {
                clean_data(cli);
            }
            "clone" => {
                clone_data(cli);
            }
            "build" => {
                let args = BuildCommandArgs {
                    no_clone: true,
                    no_clean: true,
                };
                build_database(cli, &args).await;
            }
            "match" => {
                build_nodes(cli).await;
            }
            "exit" => {
                if cli.verbose {
                    println!("Exiting...")
                }
            }
            &_ => {
                eprintln!("Invalid option"); // Should never happen
            }
        }
    }
}
