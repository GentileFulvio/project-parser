mod find_by_filesystem;
mod find_by_glob;
mod find_files;
use find_files::{find_by_string, FindParams};
use std::env;

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

pub fn find_by_filesystem() {
    let cwd = get_working_dir().unwrap();

    let packages = find_by_string(FindParams{
        location: cwd.as_str(),
        check: vec!["packages", "package.json"],
        ignore: vec!["node_modules"]
    });

    let services = find_by_string(FindParams{
        location: cwd.as_str(),
        check: vec!["services", "package.json"],
        ignore: vec!["node_modules"]
    });

    find_by_filesystem::find_by_filesystem(packages, services);
}

pub fn find_by_glob() {
    find_by_glob::search_by_glob();
}