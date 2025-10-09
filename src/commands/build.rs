use calamine::{Reader, open_workbook, Xlsx, Data};
use crate::cli::{BuildCommandArgs, Cli};
use crate::commands::clean::clean_data;
use crate::commands::clone::clone_data;

fn read_vehicles_from_excel(path: &str) {
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");

    if let Ok(range) = workbook.worksheet_range("Sheet1") {
        let header = range.rows().next().unwrap();
        header.iter().for_each(|cell| {print!("'{}', ", cell.to_string());});
        println!();

        // range.rows().for_each(|row| {
        //     row.iter().for_each(|cell| {
        //         println!("{}", cell.to_string());
        //     })
        // })
    }
}

fn read_vehicles(cli: &Cli) {
    let path = format!("{}/ved/Data/VED_Static_Data_ICE&HEV.xlsx", cli.repo_path);
    read_vehicles_from_excel(&path);
}

pub fn build_database(cli: &Cli, args: &BuildCommandArgs) {
    if !args.no_clone {
        clone_data(cli);
    }

    read_vehicles(cli);

    if !args.no_clean {
        clean_data(cli);
    }
}