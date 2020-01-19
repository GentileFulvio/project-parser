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