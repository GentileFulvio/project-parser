mod find_by_filesystem;
mod find_by_glob;
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
    find_by_filesystem::find_by_filesystem();
}

pub fn find_by_glob() {
    find_by_glob::search_by_glob();
}