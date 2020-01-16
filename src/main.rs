extern crate glob;
use std::env;
use glob::{glob, \\MatchOptions, glob_with};

fn search() {
    let pattern = format!("/services/**/package.json");
    let options = MatchOptions{
        case_sensitive: false,
        require_literal_leading_dot: false,
        require_literal_separator: false
    };

	for entry in glob_with(pattern.as_str(), options).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                match path.to_str() {
                    Some(data) => {
                        if !data.contains("node_modules") && !data.starts_with("tools") {
                            println!("{:?}", path.display())
                        }
                    },
                    None => println!("Unable to stringify buffer")
                }
            },
            Err(e) => println!("{:?}", e),
        }
    }
}

fn get_working_dir() -> Option<String> {
    match env::current_dir() {
        Ok(data) => {
            let path_string = data.to_str();

            match path_string {
                Some(cwd) => Some(cwd.to_string()),
                None => None
            }
        },
        Err(err) => {
            println!("{}", err);

            None
        }
    }
}

fn main() {
    match get_working_dir() {
        Some(cwd) => {
            println!("{}", cwd.as_str());
        },
        None => println!("Unable to retrieve current working directory")
    }

   search();
}
