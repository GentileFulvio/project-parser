extern crate glob;
use std::env;
use glob::{glob};
use std::{thread};
use std::time::Duration;

pub fn get_working_dir() -> Option<String> {
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

pub fn search_by_glob() {
    let pattern = match get_working_dir() {
        Some(str) => format!("{}/services/**/package.json", str),
        None => format!("/services/**/package.json")
    };

    for entry in glob(pattern.as_str()).expect("Failed to read glob pattern") {
        thread::sleep(Duration::from_millis(10));

        match entry {
            Ok(path) => {
                match path.to_str() {
                    Some(data) => {
                        if !data.contains("node_modules") && !data.starts_with("tools") {
                            println!("{}", data)
                        }
                    },
                    None => println!("Unable to stringify buffer")
                }
            },
            Err(e) => println!("{:?}", e),
        }
    }
}